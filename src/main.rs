use std::{
    fs::{read_to_string, write}, iter::Peekable, path::Path, str::Lines
};

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path of file which contains the copypasted clashofcode question.
    #[clap()]
    input: String,
}

#[derive(Debug, Clone)]
struct Test {
    index: u16,
    input: String,
    output: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.input);

    let contents = read_to_string(path).expect("File cannot be found");

    let mut lines = contents.trim().lines().peekable();

    let output = match *lines.peek().unwrap() {
        "The game mode is REVERSE: You do not have access to the statement. You have to guess what to do by observing the following set of tests:" => parse_reverse(lines),
        _ => unimplemented!()
    };

    write(
        args.input.split(".").next().unwrap().to_string() + ".md",
        output,
    )
    .unwrap();
}

fn parse_reverse(mut lines: Peekable<Lines>) -> String {
    let header = "*The game mode is **REVERSE:** You do not have access to the statement. You have to guess what to do by observing the following set of tests:*".to_string();

    let mut tests = vec![];

    loop {
        let line = lines.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line == "The game mode is REVERSE: You do not have access to the statement. You have to guess what to do by observing the following set of tests:" {
            continue;
        }

        if line.parse::<u16>().is_ok() {
            let test_number = line.parse::<u16>().unwrap();

            assert!(lines.next().unwrap().starts_with("Test"));
            assert!(lines.next().unwrap() == "Input");
            assert!(lines.next().unwrap() == "Expected output");

            let mut input = vec![];
            let mut output = vec![];
            let mut flag = false;

            assert!(lines.next().unwrap() == "");

            loop {
                let line = lines.next();

                if line.is_none() {
                    break;
                }

                let line = line.unwrap();

                if line == "" {
                    if !flag {
                        flag = true;
                        continue;
                    } else {
                        break;
                    }
                }

                if !flag {
                    input.push(line);
                } else {
                    output.push(line);
                }
            }

            tests.push(Test {
                index: test_number,
                input: format!("```\n{}\n```", input.join("\n")),
                output: format!("```\n{}\n```", output.join("\n")),
            });
        }
    }

    format!(
        "{header}\n\n{}",
        tests
            .iter()
            .map(|f| format!(
                "> `Test{}`\n\n__Input__\n{}\n\n__Expected output__\n{}\n",
                f.index, f.input, f.output
            ))
            .collect::<Vec<String>>()
            .join("\n")
    )
}
