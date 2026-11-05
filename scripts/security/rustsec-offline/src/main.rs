fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let db = rustsec::Database::open(std::path::Path::new(&args[1]))?;
    let lock = rustsec::Lockfile::load(&args[2])?;
    let settings = rustsec::report::Settings {
        informational_warnings: vec![rustsec::advisory::Informational::Unmaintained,
            rustsec::advisory::Informational::Unsound, rustsec::advisory::Informational::Notice],
        ..Default::default()
    };
    let report = rustsec::Report::generate(&db, &lock, &settings);
    std::fs::write(&args[3], serde_json::to_string_pretty(&report)?)?;
    println!("vulnerabilities: {}; warning categories: {}", report.vulnerabilities.count, report.warnings.len());
    if report.vulnerabilities.found { std::process::exit(1); }
    Ok(())
}
