use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="Zi Hao L.", version="0.2.0", about="Quick and dirty slides utility for text files.", long_about = None)]

pub struct SlidesArgs {
    /// Select the markdown file to be read into slides.
    #[arg(required = true)]
    pub input_file: String,
}
