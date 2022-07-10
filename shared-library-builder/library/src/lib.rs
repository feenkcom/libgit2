use libopenssl_library::libcrypto;
use shared_library_builder::{CMakeLibrary, CompiledLibraryName, GitLocation, LibraryLocation};

pub fn libgit2(binary_version: Option<impl Into<String>>) -> CMakeLibrary {
    let openssl = libcrypto(None as Option<String>);

    let libssh2 = CMakeLibrary::new(
        "ssh2",
        LibraryLocation::Git(GitLocation::github("libssh2", "libssh2").tag("libssh2-1.9.0")),
    )
    .define_common("CRYPTO_BACKEND", "OpenSSL")
    .depends(Box::new(openssl));

    CMakeLibrary::new(
        "git2",
        LibraryLocation::Git(
            GitLocation::github("syrel", "libgit2").branch("v1.1.1-windows-openssl"),
        ),
    )
    .compiled_name(CompiledLibraryName::Matching("git2".to_string()))
    .define_common("BUILD_CLAR", "OFF")
    .define_common("REGEX_BACKEND", "builtin")
    .define_common("USE_BUNDLED_ZLIB", "ON")
    .depends(Box::new(libssh2))
    .with_release_location(binary_version.map(|version| {
        LibraryLocation::Git(GitLocation::github("feenkcom", "libgit2").tag(version))
    }))
}

pub fn latest_libgit2() -> CMakeLibrary {
    let version: Option<String> = None;
    libgit2(version)
}
