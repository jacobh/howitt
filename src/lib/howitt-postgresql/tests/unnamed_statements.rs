#[test]
fn repositories_only_use_unnamed_statements() {
    // Hyperdrive with result caching disabled closes connections when the
    // driver drops named statements. Keep reads AND transactional writes on
    // the typed APIs, which send unnamed Parse/Bind/Execute in one round trip.
    let sources = [
        ("media", include_str!("../src/repos/media_repo.rs")),
        ("poi", include_str!("../src/repos/poi_repo.rs")),
        (
            "ride_points",
            include_str!("../src/repos/ride_points_repo.rs"),
        ),
        ("ride", include_str!("../src/repos/ride_repo.rs")),
        (
            "route_points",
            include_str!("../src/repos/route_points_repo.rs"),
        ),
        ("route", include_str!("../src/repos/route_repo.rs")),
        ("trip", include_str!("../src/repos/trip_repo.rs")),
        ("user", include_str!("../src/repos/user_repo.rs")),
    ];
    for (name, source) in sources {
        let compact: String = source.chars().filter(|c| !c.is_whitespace()).collect();
        for method in [
            "query",
            "query_one",
            "query_opt",
            "query_raw",
            "execute",
            "execute_raw",
            "prepare",
            "prepare_typed",
        ] {
            assert!(
                !compact.contains(&format!(".{method}(")),
                "{name} repository uses named statement API {method}"
            );
        }
    }
}
