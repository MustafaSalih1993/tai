// TODO: Better argument parsing, maybe will use a library if arguments will become too much.
use crate::common::print_usage;
const VERSION: &str = "0.0.2"; // program version

#[derive(Debug)]
pub enum Style {
    Ascii,
    Numbers,
    Blocks,
    OneChar,
    Braille,
    // TODO:
    // Colored,
}

#[derive(Debug)]
pub struct Config {
    pub image_file: String,
    pub scale: u32,
    pub dither: bool,
    pub threshold: u8,
    pub style: Style,
    pub onechar: char,
}
impl Config {
    // FIXME IM UGLY
    // Parsing arguments and return a valid config
    pub fn new(args: &mut std::env::Args) -> Option<Self> {
        let program_name = args.next().unwrap();
        // defaults
        let image_file: String;
        let mut dither: bool = false;
        let mut onechar: char = '█';
        let mut scale: u32 = 2;
        let mut style: Style = Style::Braille;
        let mut threshold: u8 = 128;

        let args: Vec<String> = args.collect();

        if args.is_empty() {
            println!("try -h | --help option to show help!");
            return None;
        }
        // loop on every argument givin
        for mut _i in 0..args.len() {
            match args[_i].as_str() {
                "-h" | "--help" => {
                    // show help.
                    print_usage(program_name);
                    return None;
                }
                "-v" | "--version" => {
                    // print program name and version and exit
                    println!("{}-v{}", program_name, VERSION);
                    return None;
                }
                "-d" | "--dither" => {
                    // modify the character when using the (--style onechar) flag;
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    dither = true;
                    _i += 1
                }
                "--onechar" => {
                    // modify the character when using the (--style onechar) flag;
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    onechar = args[_i + 1].chars().next().unwrap();
                    _i += 1
                }
                "-S" | "--style" => {
                    // art style
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    style = check_style_arg(&args[_i + 1]);
                    _i += 1
                }
                "-t" | "--threshold" => {
                    // size/scale
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    threshold = args[_i + 1].parse().unwrap_or(threshold);
                    _i += 1
                }

                "-s" | "--scale" => {
                    // size/scale
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    scale = args[_i + 1].parse().unwrap_or(scale);
                    _i += 1
                }
                _ => {
                    continue;
                }
            }
        }
        //args loop ends here

        if args[args.len() - 1].starts_with('-') {
            return None;
        };

        image_file = args.into_iter().last().unwrap();

        //returning
        Some(Self {
            image_file,
            scale,
            dither,
            threshold,
            style,
            onechar,
        })
    }
}

fn check_style_arg(arg: &str) -> Style {
    match arg {
        "ascii" => Style::Ascii,
        "blocks" => Style::Blocks,
        "braille" => Style::Braille,
        "numbers" => Style::Numbers,
        "onechar" => Style::OneChar,
        _ => Style::Braille,
    }
}
