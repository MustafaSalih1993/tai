use crate::{Config, Style};
use getopts::Options;
const VERSION: &str = "0.0.8"; // program version

pub fn parse(args: Vec<String>) -> Option<Config> {
    // defaults
    let mut config = Config::default();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this help message");
    opts.optflag("b","background","Will apply the colors on the \"background\" of the characters instead of coloring the foreground");
    opts.optflag("c", "colored", "Will return true colored(RGB) art");
    opts.optflag("d", "dither", "enables image dithering");
    opts.optopt(
        "o",
        "onechar",
        "Followed by a character, This will modify the default character used by (-S onechar)",
        "CHARACTER",
    );
    opts.optflag(
        "N",
        "no-scale",
        "will keep the original size of the image, default to false",
    );
    opts.optopt(
        "D",
        "dither-scale",
        "used with \"-d\" option, controls the scale number for the dithering, default to 16",
        "NUMBER",
    );
    opts.optflag(
        "O",
        "once",
        "Will play the image's animation only once (no looping)",
    );
    opts.optopt(
        "S",
        "style",
        "Followed by one of: {{ascii, numbers, blocks, onechar, braille}}, default to \"braille\"",
        "STYLE",
    );
    opts.optopt("","sleep","Followed by number, controls the sleep delay(milli seconds) between animation frames. default to 100","MILLI_SECONDS");
    opts.optopt(
        "s",
        "scale",
        "Followed by a number to Resize the output (lower number means bigger output) default to 2",
        "NUMBER",
    );
    opts.optopt(
        "t",
        "table",
        "Make a custom ascii table,(works only with \"ascii\" Style) seperated by ','\n\
                            \t\t\t\t ex: tai -S ascii --table \" ,.,:,x,@\" image.png",
        "TABLE_OF_CHARACTERS",
    );
    opts.optflag("v", "version", "Print tai's Version and exit!");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => std::panic::panic_any(f.to_string()),
    };

    if matches.opt_present("h") {
        print!(
            "{}",
            opts.usage(&format!("USAGE: {} [Options] IMAGE", program))
        );
        return None;
    }

    if matches.opt_present("v") {
        println!("{} v{}", program, VERSION);
        return None;
    }

    if matches.opt_present("colored") {
        config.colored = true;
    }
    if matches.opt_present("background") {
        if !config.colored {
            eprintln!(
                "--colored is not enabled to use this option, for now i will enable it for you"
            );
            config.colored = true;
        }
        // 48==applying the color on the background of the char,
        // 38(default)==applying the color on the foreground.
        config.background = 48;
    }
    if matches.opt_present("d") {
        config.dither = true;
    }
    if matches.opt_present("D") {
        let dither_scale = matches.opt_get::<u8>("D").unwrap().unwrap();
        if !config.dither {
            eprintln!("image dithering is not enabled, for now i will enable it for you");
            config.dither = true;
        }
        config.dither_scale = dither_scale;
    }
    if matches.opt_present("N") {
        config.original_size = true;
    }
    if matches.opt_present("S") {
        let style = matches.opt_str("S").unwrap();
        config.style = check_style_arg(&style);
    }
    if matches.opt_present("onechar") {
        let onechar = matches.opt_get::<char>("onechar").unwrap().unwrap();
        match config.style {
            Style::OneChar => {}
            _ => {
                eprintln!("this option only works with onechar style (-S onechar), for now i will switch it for you");
                config.style = Style::OneChar;
            }
        }
        config.onechar = onechar;
    }
    if matches.opt_present("once") {
        config.once = true;
    }

    if matches.opt_present("sleep") {
        let sleep = matches.opt_get::<u64>("sleep").unwrap().unwrap();
        config.sleep = sleep;
    }
    if matches.opt_present("scale") {
        let scale = matches.opt_get::<u32>("scale").unwrap().unwrap();
        config.scale = scale;
    }
    if matches.opt_present("table") {
        let table = matches
            .opt_str("table")
            .unwrap()
            .split(',')
            .map(|token| token.trim().chars().next().unwrap_or(' '))
            .collect::<Vec<char>>();
        config.table = table;
        match config.style {
            Style::Ascii => {}
            _ => {
                eprintln!("--table option works only with ascii style(-S/--style ascii), for now i will enable it for you");
                config.style = Style::Ascii
            }
        }
    }
    if !matches.free.is_empty() {
        config.image_file = matches.free[0].to_string();
    };

    Some(config)
}

fn check_style_arg(arg: &str) -> Style {
    match arg {
        "ascii" => Style::Ascii,
        "blocks" => Style::Blocks,
        "braille" => Style::Braille,
        "numbers" => Style::Numbers,
        "onechar" => Style::OneChar,
        _ => Style::default(),
    }
}
