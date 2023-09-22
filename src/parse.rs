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

#[derive(Default)]
pub struct Slideshow {
    pub slides: Vec<Slide>,
    pub index: usize,
}

impl Slideshow {
    pub fn new() -> Self {
        Slideshow::default()
    }

    pub fn from_path(input_file: &String) -> Result<Self> {
        let input_file = File::open(input_file)?;
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
        Ok(slideshow)
    }

    pub fn from_cli() -> Result<Self> {
        let args = SlidesArgs::parse();
        Ok(Slideshow::from_path(&args.input_file)?)
    }
}
