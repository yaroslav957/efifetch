use std::error::Error;

use vergen::{BuildBuilder, Emitter, RustcBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let rustc = RustcBuilder::default().semver(true).build()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
