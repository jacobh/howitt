use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let directory = manifest.join("migrations");
    println!("cargo:rerun-if-changed={}", directory.display());

    let mut migrations = fs::read_dir(&directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".sql"))
        .map(|name| {
            let (version, description) = name
                .strip_prefix('V')
                .and_then(|name| name.strip_suffix(".sql"))
                .and_then(|name| name.split_once("__"))
                .unwrap_or_else(|| panic!("invalid migration filename: {name}"));
            let version = version
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("invalid migration version: {name}"));
            (version, description.to_owned(), name)
        })
        .collect::<Vec<_>>();
    migrations.sort_by_key(|migration| migration.0);

    for pair in migrations.windows(2) {
        assert_ne!(pair[0].0, pair[1].0, "duplicate migration version");
    }

    let entries = migrations
        .iter()
        .map(|(version, description, filename)| {
            format!(
                "Migration {{ version: {version}, name: {description:?}, sql: include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/migrations/{filename}\")) }}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("migrations.rs"),
        format!("pub static BUNDLED_MIGRATIONS: &[Migration] = &[\n{entries}\n];\n"),
    )
    .unwrap();
}
