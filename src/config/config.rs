// TODO: Better argument parsing
use crate::config::argument_parsing;

#[derive(Debug)]
pub enum Style {
    Ascii,
    Blocks,
    Braille,
    Numbers,
    OneChar,
}

#[derive(Debug)]
pub struct Config {
    pub background: u8,
    pub colored: bool,
    pub dither: bool,
    pub image_file: String,
    pub onechar: char,
    pub scale: u32,
    pub sleep: u64,
    pub style: Style,
    pub threshold: u8,
}
impl Config {
    // FIXME IM UGLY YOU ASSHOLE!
    // Parsing arguments and return a valid config
    pub fn new(args: &mut std::env::Args) -> Option<Self> {
        // getting rid of the first arg (program name)
        args.next().unwrap();
        // converting from iterator to vector (to access the indexes).
        let args: Vec<String> = args.collect();

        argument_parsing::parse(args)
    }
}
