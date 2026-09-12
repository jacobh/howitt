fn main() {
    let start = std::time::Instant::now();
    let fuzzy = tzf_rs::FuzzyFinder::new();
    println!("fuzzy initialization={:?}", start.elapsed());
    for (lng, lat) in [
        (144.96, -37.81),
        (141.0, -33.0),
        (129.0, -25.0),
        (153.5, -28.16),
        (0.0, 90.0),
    ] {
        println!("{lng},{lat}: {}", fuzzy.get_tz_name(lng, lat));
    }
    let finder = tzf_rs::Finder::new();
    println!(
        "timezone={} total initialization={:?}",
        finder.get_tz_name(141.0, -33.0),
        start.elapsed()
    );
}
