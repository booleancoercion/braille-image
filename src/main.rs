use clap::App;
use clap::Arg;

use std::fs::OpenOptions;
use std::io::prelude::*;

use braille_image::ProgramOptions;

fn main() {
    let matches = App::new("Braille Art Generator")
        .version("0.1.1")
        .author("boolean_coercion <booleancoercion@gmail.com>")
        .about("Generates Unicode art from images using braille characters.")
        .arg(
            Arg::new("INPUT")
                .long_about("Sets the input file to use")
                .required_unless_present("use-existing")
        )
        .arg(
            Arg::new("use-existing")
                .long("use-existing")
                .required(false)
                .long_about("Uses the existing canny dump generated with --debug instead of calculating it again.
This is useful if you want to tinker with the settings and not have it take a long time every run.
INPUT must not be present with this flag.")
                .conflicts_with_all(&["INPUT", "threshold", "debug", "sigma"])
        )
        .arg(
            Arg::new("scale")
                .short('s')
                .long("scale")
                .long_about("Sets the scale factor to use with the image. Must be a positive real number.")
                .default_value("1")
                .takes_value(true),
        )
        .arg(
            Arg::new("invert")
                .short('i')
                .long("invert")
                .long_about("Inverts the image. This is used when you want to fill the bright spaces with dots instead of the dark ones.")
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .long_about("Specifies the brightness threshold that determines whether a pixel will be converted to a dot or not.
Must be an integer between 0 and 255.

When used with --canny, this instead becomes the weak threshold, which must be between 0.0 and 1.0 .")
                .default_value_if("canny", None, "0.1")
                .default_value("127")
                .takes_value(true)
        )
        .arg(
            Arg::new("debug")
                .long("debug")
        )
        .arg(
            Arg::new("sigma")
                .long("sigma")
                .required(false)
                .default_value("4.0")
                .long_about("Sets the sigma value to use with canny edge-detection. Recommended values are between 3.0 and 6.0.")
        )
        .arg(
            Arg::new("canny")
                .long("canny")
                .long_about("Specifies that the image should be processed using canny edge-detection with built-in/provided parameters.")
                .required(false)
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap_or("canny.png");
    let scale: f32 = matches
        .value_of("scale")
        .unwrap()
        .parse()
        .unwrap_or_exit("scale must be a positive real number.", -1);
    if scale <= 0.0 {
        eprintln!("scale must be a positive real number.");
        std::process::exit(-1);
    }

    let invert = matches.is_present("invert");
    let canny = matches.is_present("canny");
    let threshold: f32 = matches
        .value_of("threshold")
        .unwrap()
        .parse()
        .unwrap_or_exit("invalid threshold", -2);

    if canny {
        if threshold <= 0.0 || threshold >= 1.0 {
            eprintln!("invalid threshold for canny");
            std::process::exit(-3);
        }
    } else if !(1.0..=254.0).contains(&threshold) {
        eprintln!("invalid threshold for brightness");
        std::process::exit(-3);
    }
    let debug = matches.is_present("debug");

    let sigma: f32 = matches
        .value_of("sigma")
        .unwrap()
        .parse()
        .unwrap_or_exit("invalid sigma", -4);

    let use_existing = matches.is_present("use-existing");

    let img = braille_image::get_image(filename)
        .unwrap_or_exit("Couldn't find file or file is not an image.", -5);

    let result = braille_image::run(
        img,
        ProgramOptions {
            filename: filename.to_owned(),
            scale,
            invert,
            threshold,
            canny,
            debug,
            sigma,
            use_existing,
        },
    );

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("output.txt")
        .unwrap_or_exit("Couldn't open output.txt", -3);

    writeln!(output_file, "{}", result).unwrap_or_exit("Couldn't write to output.txt", -4);
}

trait UnwrapExit<T> {
    fn unwrap_or_exit(self, err: &str, code: i32) -> T;
}

impl<T> UnwrapExit<T> for Option<T> {
    fn unwrap_or_exit(self, err: &str, code: i32) -> T {
        if let Some(x) = self {
            x
        } else {
            eprintln!("{}", err);
            std::process::exit(code);
        }
    }
}

impl<T, E> UnwrapExit<T> for Result<T, E> {
    fn unwrap_or_exit(self, err: &str, code: i32) -> T {
        if let Ok(x) = self {
            x
        } else {
            eprintln!("{}", err);
            std::process::exit(code);
        }
    }
}
