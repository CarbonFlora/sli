use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

use crate::args::SlidesArgs;

#[derive(Clone)]
pub struct Slide {
    pub lines: Vec<String>,
}

impl Slide {
    pub fn new() -> Self {
        Slide { lines: Vec::new() }
    }
}

pub struct Slideshow {
    pub slides: Vec<Slide>,
}

impl Slideshow {
    pub fn new() -> Self {
        Slideshow { slides: Vec::new() }
    }
}

pub fn slides() -> Result<()> {
    let args = SlidesArgs::parse();
    let input_file = File::open(args.input_file)?;
    let mut slide = Slide::new();
    let mut slideshow = Slideshow::new();

    for line in BufReader::new(input_file)
        .lines()
        .map(|x| x.unwrap_or_default())
    {
        if line.starts_with("---") {
            slideshow.slides.push(slide.clone());
            slide = Slide::new();
            continue;
        }
        slide.lines.push(line);
    }

    Ok(())
}
