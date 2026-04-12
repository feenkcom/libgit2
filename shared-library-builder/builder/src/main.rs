use std::error::Error;

use shared_library_builder::{build_standalone, build_third_party};

use libgit2_library::latest_libgit2;

fn main() -> Result<(), Box<dyn Error>> {
    build_third_party(|_| Ok(Box::new(latest_libgit2())))
}
