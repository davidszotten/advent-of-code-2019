use clap::{App, Arg};
use failure::{bail, Error};
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};
use std::result;

pub type Result<T> = result::Result<T, Error>;

enum Part {
    Part1,
    Part2,
}

enum Source {
    Stdin,
    File(String),
}

struct Args {
    part: Part,
    source: Source,
}

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_file(filename: &str) -> Result<String> {
    let mut buffer = String::new();
    let mut handle = File::open(filename)?;

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_input() -> Result<Args> {
    let matches = App::new("adventofcode")
        .arg(
            Arg::with_name("part")
                .short("p")
                .takes_value(true)
                .default_value("1")
                .possible_values(&["1", "2"]),
        )
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use, or `-` for stdin")
                .required(true)
                .index(1),
        )
        .get_matches();

    let part = match matches.value_of("part").unwrap_or("1") {
        "1" => Part::Part1,
        "2" => Part::Part2,
        _ => bail!("Invalid part"),
    };
    let source = match matches
        .value_of("input")
        .expect("input is required but missing")
    {
        "-" => Source::Stdin,
        filename => Source::File(filename.into()),
    };
    Ok(Args { part, source })
}

type DayFunc<T> = dyn Fn(&str) -> Result<T>;

fn run<S, T>(part1: &DayFunc<S>, part2: &DayFunc<T>) -> Result<String>
where
    S: Display,
    T: Display,
{
    let args = parse_input()?;
    let input = match args.source {
        Source::Stdin => read_stdin(),
        Source::File(filename) => read_file(&filename),
    }?;
    match args.part {
        Part::Part1 => part1(&input).map(|res| format!("{}", res)),
        Part::Part2 => part2(&input).map(|res| format!("{}", res)),
    }
}

pub fn dispatch<S, T>(part1: &DayFunc<S>, part2: &DayFunc<T>) -> Result<()>
where
    S: Display,
    T: Display,
{
    let result = run(part1, part2)?;
    println!("{}", result);
    Ok(())
}
