use clap::{load_yaml, App};

use std::fs::OpenOptions;
use std::io::prelude::*;

use braille_image::ProgramOptions;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

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
