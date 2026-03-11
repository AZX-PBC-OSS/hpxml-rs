macro_rules! fixture {
    ($path:expr) => {
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/", $path))
    };
}
