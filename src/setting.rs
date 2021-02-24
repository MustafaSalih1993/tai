// TODO: Better argument parsing, maybe will use a library if arguments will become too much.

const VERSION: &str = "0.0.1"; // program version

#[derive(Debug)]
pub enum Style {
    Ascii,
    Numbers,
    Blocks,
    OneChar,
    // TODO:
    // Braille,
}

#[derive(Debug)]
pub struct Settings {
    pub image_file: String,
    pub image_size: u32,
    pub style: Style,
    pub onechar: char,
}
impl Settings {
    pub fn new(args: &mut std::env::Args) -> Option<Self> {
        let program_name = args.next().unwrap();
        let image_file: String;
        let mut image_size: u32 = 16;
        let mut style: Style = Style::Ascii;
        let mut onechar: char = '█';

        let args: Vec<String> = args.collect();

        if args.len() < 1 {
            println!("try -h | --help option to show help!");
            return None;
        }

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
                "--onechar" => {
                    // modify the character when using the (--style onechar) flag;
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    onechar = args[_i + 1].chars().nth(0).unwrap();
                    _i += 1
                }
                "-S" | "--style" => {
                    // art style
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    style = check_style(&args[_i + 1]);
                    _i += 1
                }

                "-s" | "--size" => {
                    // size/scale
                    if _i == args.len() - 1 {
                        print_usage(program_name);
                        return None;
                    };
                    image_size = args[_i + 1].parse().unwrap_or(image_size);
                    _i += 1
                }
                _ => {
                    continue;
                }
            }
        }

        if args[args.len() - 1].starts_with("-") {
            return None;
        };

        image_file = args.into_iter().last().unwrap();

        Some(Self {
            image_file,
            image_size,
            style,
            onechar,
        })
    }
}
pub fn print_usage(program_name: String) {
    println!("USAGE: {} [OPTIONS] [IMAGE_FILE]", program_name);
    println!();
    println!("OPTIONS: ");
    println!("\t -h | --help\t Show this help message");
    println!("\t -s | --size\t Followed by a number to Resize the output (lower number means bigger output)");
    println!("\t -S | --style\t Followed by one of: {{ascii, numbers, blocks, onechar}}");
    println!("\t --onechar\t Followed by a character, This will modify the character when using (-S onechar)");
    println!(
        "\t -v | --version\t Print {}'s Version and exit!.",
        program_name
    );
}

fn check_style(arg: &String) -> Style {
    match arg.as_str() {
        "ascii" => Style::Ascii,
        "numbers" => Style::Numbers,
        "blocks" => Style::Blocks,
        "onechar" => Style::OneChar,
        _ => Style::Ascii,
        //TODO
    }
}
