use crate::arguments::argument_parsing;
/* TODO:
 - Better argument parsing
 - Implement Defaults to Style
*/

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
    pub dither_scale: u8,
    pub image_file: String,
    pub onechar: char,
    pub scale: u32,
    pub sleep: u64,
    pub style: Style,
    pub threshold: u8,
    pub table: Vec<char>,
}
impl Config {
    // Parsing arguments and return a valid config
    pub fn new(args: &mut std::env::Args) -> Option<Self> {
        // getting rid of the first arg (program name)
        args.next().unwrap();
        // converting from iterator to vector (to access the indexes).
        let args: Vec<String> = args.collect();

        argument_parsing::parse(args)
    }
}
