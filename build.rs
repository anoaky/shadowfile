fn main() {
    // Ensure target is not Windows.
    std::env::var("CARGO_CFG_UNIX").expect("This crate does not support non-Unix targets.");

    // If *host* is a Mac, ensure minimum required version.
    // `target_os` in build scripts is the OS of the host, not the compilation target.
    #[cfg(target_os = "macos")]
    {
        use os_info::Version;

        let version = os_info::get().version().clone();
        // First version of macOS on which I could find documentation on `hdiutil`
        let min_version = Version::from_string("10.13.6");
        // Since we conditionally compile on macOS, we don't need to check the structure of `version`.
        if version < min_version {
            panic!("Building this crate requires macOS >= 10.13.6.");
        }
    }
}
