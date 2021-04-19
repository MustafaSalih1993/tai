use crate::{utils::print_usage, Config, Style};

const VERSION: &str = "0.0.5"; // program version

pub fn parse(args: Vec<String>) -> Option<Config> {
    // defaults
    let mut config = Config::default();

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

                config.background = 48;
            }
            "--colored" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                config.colored = true;
            }

            "-d" | "--dither" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                config.dither = true;
                _i += 1
            }
            "-D" | "--dither-scale" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                let input = args[_i + 1].parse::<u8>();
                config.dither_scale = if input.is_err() || input.unwrap() < 1 {
                    eprintln!("Error: invalid dither-scale value. using defaults!");
                    config.dither_scale
                } else {
                    args[_i + 1].parse().unwrap_or(config.dither_scale)
                };
                _i += 1;
            }
            "-N" | "--no-scale" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                config.original_size = true;
                _i += 1
            }
            "--onechar" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                config.onechar = args[_i + 1].chars().next().unwrap();
                _i += 1
            }
            "-S" | "--style" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                config.style = check_style_arg(&args[_i + 1]);
                _i += 1
            }
            "--sleep" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };

                config.sleep = args[_i + 1].parse().unwrap_or(config.sleep);
                _i += 1
            }
            "-s" | "--scale" => {
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                let input = args[_i + 1].parse::<u32>();
                config.scale = if input.is_err() || input.unwrap() < 1 {
                    eprintln!("Error: invalid scale number using defaults!");
                    config.scale
                } else {
                    args[_i + 1].parse().unwrap_or(config.scale)
                };
                _i += 1;
            }
            "--table" => {
                // custom ascii table
                if _i == args.len() - 1 {
                    print_usage();
                    return None;
                };
                config.table = args[_i + 1]
                    .parse::<String>()
                    .unwrap_or_else(|_| "".to_string())
                    .split(',')
                    .map(|token| token.trim().chars().next().unwrap_or(' '))
                    .collect::<Vec<char>>();
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

    config.image_file = args.into_iter().last().unwrap();

    Some(config)
}

fn check_style_arg(arg: &str) -> Style {
    match arg {
        "ascii" => Style::Ascii,
        "blocks" => Style::Blocks,
        "braille" => Style::Braille,
        "numbers" => Style::Numbers,
        "onechar" => Style::OneChar,
        _ => {
            eprintln!("Error: Unknown style. using defaults");
            Style::default()
        }
    }
}
