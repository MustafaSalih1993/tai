use crate::{utils::print_usage, Config, Style};

const VERSION: &str = "0.0.3"; // program version

pub fn parse(args: Vec<String>) -> Option<Config> {
    // defaults
    let image_file: String;
    let mut background: u8 = 38;
    let mut colored: bool = false;
    let mut dither: bool = false;
    let mut onechar: char = '█';
    let mut scale: u32 = 2;
    let mut sleep: u64 = 100;
    let mut style: Style = Style::Braille;
    let mut threshold: u8 = 128;

    if args.is_empty() {
        println!("try -h | --help option to show help!");
        return None;
    }
    // loop on every argument givin
    for mut _i in 0..args.len() {
        match args[_i].as_str() {
            "-h" | "--help" => {
                // show help.
                print_usage();
                return None;
            }
            // 48==applying the color on the background of the char,
            // 38(default)==applying the color on the foreground.
            "--background" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                background = 48;
            }
            "--colored" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                colored = true;
            }

            "-d" | "--dither" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                dither = true;
                _i += 1
            }
            "--onechar" => {
                // modify the character when using the (--style onechar) flag;
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                onechar = args[_i + 1].chars().next().unwrap();
                _i += 1
            }
            "-S" | "--style" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                style = check_style_arg(&args[_i + 1]);
                _i += 1
            }
            "--sleep" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                sleep = args[_i + 1].parse().unwrap_or(sleep);
                _i += 1
            }
            "-s" | "--scale" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                scale = args[_i + 1].parse().unwrap_or(scale);
                _i += 1
            }
            "-t" | "--threshold" => {
                // threshold
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                threshold = args[_i + 1].parse().unwrap_or(threshold);
                _i += 1
            }
            "-v" | "--version" => {
                // print program name and version and exit
                println!("tai-v{}", VERSION);
                return None;
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
    Some(Config {
        background,
        colored,
        dither,
        image_file,
        onechar,
        scale,
        sleep,
        style,
        threshold,
    })
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
