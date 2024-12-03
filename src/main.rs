use std::{
    fs::{read_to_string, write},
    iter::Peekable,
    path::Path,
    str::Lines,
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
        "Goal" => parse_fastest(lines),
        _ => unimplemented!()
    };

    write(
        args.input.split(".").next().unwrap().to_string() + ".md",
        output,
    )
    .unwrap();
}

fn parse_fastest(mut lines: Peekable<Lines>) -> String {
    let header = "**Goal**";

    lines.next().unwrap();

    fn consume<'a>(
        mut lines: Peekable<Lines<'a>>,
        stop_at: &'a str,
    ) -> (Peekable<Lines<'a>>, String) {
        let mut output = String::new();

        loop {
            if lines.peek().is_none() {
                break;
            }

            let line = *lines.peek().unwrap();

            if line == stop_at {
                lines.next().unwrap();
                break;
            }

            lines.next().unwrap();

            if line.is_empty() {
                continue;
            }

            let line = line.replace("*", r"\*");

            output += &(line.trim().to_owned() + "\n");
        }

        (lines, output)
    }

    let (lines, question) = consume(lines, "Input");

    let (lines, input) = consume(lines, "Output");

    let (lines, output) = consume(lines, "Constraints");

    let (mut lines, constraints) = consume(lines, "Example");

    let mut example_input = String::new();
    let mut example_output = String::new();
    let mut flag = false;

    assert!(lines.next().unwrap() == "Input");

    loop {
        let line = lines.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line == "Output" {
            if !flag {
                flag = true;
                continue;
            } else {
                break;
            }
        }

        if line.is_empty() {
            continue;
        }

        if !flag {
            example_input += &(line.to_owned() + "\n");
        } else {
            example_output += &(line.to_owned() + "\n");
        }
    }

    format!(
        "{header}\n\n{}\n\n`Input`\n```\n{input}```\n\n`Output`\n```\n{output}```\n\n`Constraints`\n```\n{constraints}```\n\n> Example\n\n`Input`\n```\n{example_input}```\n\n`Output`\n```\n{example_output}```",
        question.lines().map(|f| format!("> {f}")).collect::<Vec<String>>().join("\n")
    )
}

fn parse_reverse(mut lines: Peekable<Lines>) -> String {
    let header = "*The game mode is **REVERSE:** You do not have access to the statement. You have to guess what to do by observing the following set of tests:*".to_string();
    lines.next().unwrap();

    let mut tests = vec![];

    loop {
        let line = lines.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

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
