use clap::App;
use clap::Arg;

use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let matches = App::new("Braille Art Generator")
        .version("0.1.0")
        .author("boolean_coercion <booleancoercion@gmail.com>")
        .about("Generates Unicode art from images using braille characters.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .help("Sets the scale factor to use with the image. Must be a positive real number.")
                .default_value("1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("invert")
                .short("i")
                .long("invert")
                .help("Inverts the image. This is used when you want to fill the bright spaces with dots instead of the dark ones.")
        )
        .arg(
            Arg::with_name("threshold")
                .short("t")
                .long("threshold")
                .help("Specifies the brightness threshold that determines whether a pixel will be converted to a dot or not.
 Must be an integer between 0 and 255.")
                .default_value("127")
                .takes_value(true)
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let scale: f64 = matches
        .value_of("scale")
        .unwrap()
        .parse()
        .unwrap_or_exit("scale must be a positive real number.", -1);
    if scale <= 0.0 {
        eprintln!("scale must be a positive real number.");
        std::process::exit(-1);
    }

    let invert = matches.is_present("invert");
    let threshold: u8 = matches
        .value_of("threshold")
        .unwrap()
        .parse()
        .unwrap_or_exit("threshold must be an integer between 0 and 255.", -2);

    let img = braille_image::get_image(filename)
        .unwrap_or_exit("Couldn't find file or file is not an image.", -2);

    let result = braille_image::run(img, scale, invert, threshold);

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
