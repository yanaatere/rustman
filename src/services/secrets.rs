//! Redaction boundary for diagnostic output. This never mutates request data.
#[derive(Default)]
pub struct Redactor {
    values: Vec<String>,
}

impl Redactor {
    pub fn add(&mut self, value: &str) {
        if !value.is_empty() && !self.values.iter().any(|s| s == value) {
            self.values.push(value.to_owned());
            // Longest first prevents a short overlapping value exposing suffixes.
            self.values.sort_by_key(|v| std::cmp::Reverse(v.len()));
        }
    }
    pub fn add_header(&mut self, name: &str, value: &str) {
        if !crate::services::variables::sensitive(name) {
            return;
        }
        self.add(value);
        if name.eq_ignore_ascii_case("cookie") {
            for part in value.split(';') {
                if let Some((_, secret)) = part.split_once('=') {
                    self.add(secret.trim());
                }
            }
        } else if name.eq_ignore_ascii_case("set-cookie") {
            for line in value.lines() {
                if let Some((_, secret)) = line.split(';').next().unwrap_or("").split_once('=') {
                    self.add(secret.trim());
                }
            }
        } else if name.eq_ignore_ascii_case("authorization")
            || name.eq_ignore_ascii_case("proxy-authorization")
        {
            if let Some((_, secret)) = value.split_once(' ') {
                self.add(secret.trim());
            }
        }
    }
    pub fn redact(&self, text: &str) -> String {
        let mut safe = text.to_owned();
        for secret in &self.values {
            safe = safe.replace(secret, crate::services::variables::MASK);
        }
        safe
    }
}
/// URL presentation only. Request/history records retain their original bytes.
/// Bound processing before parsing so a giant imported URL cannot stall layout.
pub fn display_url(url: &str) -> String {
    use crate::services::variables::{MASK, sensitive};
    if url.len() > crate::consts::LABEL_RENDER_LIMIT {
        return "[Long URL hidden; open request to inspect]".into();
    }
    let (without_fragment, fragment) = url
        .split_once('#')
        .map_or((url, None), |(a, b)| (a, Some(b)));
    let (base, query) = without_fragment
        .split_once('?')
        .map_or((without_fragment, None), |(a, b)| (a, Some(b)));
    let mut shown = base.to_owned();
    let authority_start = base.find("://").map_or(0, |i| i + 3);
    let authority_end = base[authority_start..]
        .find('/')
        .map_or(base.len(), |i| authority_start + i);
    if let Some(at) = base[authority_start..authority_end].rfind('@') {
        shown.replace_range(authority_start..authority_start + at, MASK);
    }
    if let Some(query) = query {
        shown.push('?');
        let parts: Vec<String> = query
            .split('&')
            .map(|part| {
                let Some((raw_key, _)) = part.split_once('=') else {
                    return part.to_owned();
                };
                let key = reqwest::Url::parse(&format!("https://display.invalid/?{part}"))
                    .ok()
                    .and_then(|u| u.query_pairs().next().map(|(k, _)| k.into_owned()))
                    .unwrap_or_else(|| raw_key.to_owned());
                if sensitive(&key) {
                    format!("{raw_key}={MASK}")
                } else {
                    part.to_owned()
                }
            })
            .collect();
        shown.push_str(&parts.join("&"));
    }
    if fragment.is_some_and(|s| !s.is_empty()) {
        shown.push('#');
        shown.push_str(MASK);
    }
    shown
}

/// Known credential-named JSON fields in automatically generated previews.
/// Not a universal secret detector; opaque/non-JSON bodies need separate policy.
pub fn redact_json_fields(body: &str) -> String {
    fn walk(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(fields) => {
                for (key, value) in fields {
                    if crate::services::variables::sensitive(key) {
                        *value = serde_json::Value::String(crate::services::variables::MASK.into());
                    } else {
                        walk(value);
                    }
                }
            }
            serde_json::Value::Array(values) => {
                for value in values {
                    walk(value);
                }
            }
            _ => {}
        }
    }
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(mut value) => {
            walk(&mut value);
            value.to_string()
        }
        Err(_) => body.to_owned(),
    }
}
