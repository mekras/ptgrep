#[macro_use]
extern crate clap;
extern crate exitcode;
extern crate regex;

use clap::App;
use clap::Values;
use regex::Regex;
use std::process::Command;
use std::process::exit;
use std::string::String;
use ::ThresholdKind::{Lower, Higher};

/// Виды пороговых значений.
#[derive(Copy, Clone)]
enum ThresholdKind {
    Lower,
    Higher,
}


/// Пороговое значение.
struct Threshold {
    /// Имя параметра.
    parameter: String,
    /// Вид порога.
    kind: ThresholdKind,
    /// Пороговое значение.
    value: f32,
}

impl Threshold {
    fn satisfied(&self, value: f32) -> bool {
        match self.kind {
            Lower => value >= self.value,
            Higher => value <= self.value,
        }
    }
}

///
/// Производит разбор пороговых аргументов.
///
/// - kind — вид пороговых значений
/// - values — значения аргументов для этого вида порогов из командной строки
///
fn parse_threshold_args(kind: ThresholdKind, values: Values) -> Vec<Threshold> {
    let mut thresholds: Vec<Threshold> = vec![];

    for value in values {
        let parts: Vec<&str> = value.split("=").collect();
        thresholds.push(
            Threshold {
                parameter: parts[0].to_string(),
                kind,
                value: match parts[1].parse::<f32>() {
                    Ok(value) => value,
                    Err(error) => {
                        println!(
                            "{} value should be a float: {}.",
                            parts[0].to_string(),
                            error
                        );
                        exit(exitcode::DATAERR);
                    }
                },
            }
        );
    }

    return thresholds;
}

///
/// Обрабатывает вывод выполненной команды.
///
/// Возвращает список найденных проблем.
///
fn process_output(output: &String, pattern: Regex, thresholds: Vec<Threshold>) -> Vec<String> {
    // Список найденных проблем.
    let mut failures: Vec<String> = vec![];

    for matches in pattern.captures_iter(output.as_str()) {
        for threshold in &thresholds {
            let matched = matches.name(threshold.parameter.as_str());
            if matched == None {
                continue;
            }

            let value = match matched.unwrap().as_str().parse::<f32>() {
                Ok(value) => value,
                Err(error) => {
                    println!(
                        "Matched value for {} should be a float: {}.",
                        threshold.parameter.as_str(),
                        error
                    );
                    exit(exitcode::DATAERR);
                }
            };

            if !threshold.satisfied(value) {
                failures.push(
                    format!(
                        "{} value {} is {} than {}",
                        threshold.parameter,
                        value,
                        match threshold.kind {
                            Lower => "lower",
                            Higher => "higher",
                        },
                        threshold.value
                    )
                );
            }
        }
    }

    return failures;
}

///
/// Главная функция.
///
fn main() {
    let yaml = load_yaml!("cli.yml");
    let arguments = App::from_yaml(yaml).get_matches();

    let mut command = arguments.values_of("COMMAND").unwrap();
    let pattern = arguments.value_of("pattern").unwrap();
    let regex = Regex::new(pattern).unwrap();

    // Список пороговых значений.
    let mut thresholds: Vec<Threshold> = vec![];

    if arguments.is_present("lower") {
        for argument in arguments.values_of("lower") {
            thresholds.append(&mut parse_threshold_args(Lower, argument));
        }
    }

    if arguments.is_present("higher") {
        for argument in arguments.values_of("higher") {
            thresholds.append(&mut parse_threshold_args(Higher, argument));
        }
    }

    let process = match Command::new(command.next().unwrap())
        .args(command)
        .output() {
        Ok(process) => process,
        Err(error) => panic!("Running process error: {}", error),
    };

    let output = String::from_utf8(process.stdout).unwrap();

    let failures: Vec<String> = process_output(&output, regex, thresholds);

    println!("{}", output.as_str());

    if failures.len() > 0 {
        println!("Threshold failures:");
        for failure in failures {
            println!(" ⋅ {}", failure);
        }
        exit(exitcode::DATAERR);
    }
}
