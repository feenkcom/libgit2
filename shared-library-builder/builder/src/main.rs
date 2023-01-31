use std::error::Error;

use shared_library_builder::build_standalone;

use libgit2_library::latest_libgit2;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libgit2())))
}
