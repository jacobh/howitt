fn main() {
    let start = std::time::Instant::now();
    let finder = tzf_rs::DefaultFinder::new();
    println!("initialization={:?}", start.elapsed());
    for (lng, lat) in [
        (144.96, -37.81),
        (141.0, -33.0),
        (129.0, -25.0),
        (153.5, -28.16),
        (0.0, 90.0),
    ] {
        println!("{lng},{lat}: {}", finder.get_tz_name(lng, lat));
    }
    println!(
        "timezone={} total elapsed={:?}",
        finder.get_tz_name(141.0, -33.0),
        start.elapsed()
    );
}
