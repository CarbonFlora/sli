use anyhow::Result;
use clap::Parser;

use crate::args::SlidesArgs;

pub fn slides() -> Result<()> {
    let args = SlidesArgs::parse();

    Ok(())
}
