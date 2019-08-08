#[macro_use]
extern crate clap;
extern crate exitcode;
extern crate regex;

use clap::App;
use clap::Values as InputValues;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::{Command, ExitStatus};
use std::process::exit;
use std::string::String;

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
            ThresholdKind::Lower => value >= self.value,
            ThresholdKind::Higher => value <= self.value,
        }
    }
}

///
/// Производит разбор пороговых аргументов.
///
/// - kind — вид пороговых значений
/// - values — значения аргументов для этого вида порогов из командной строки
///
fn parse_threshold_args(kind: ThresholdKind, values: InputValues) -> Vec<Threshold> {
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
                            "Cannot convert {} value \"{}\" to float: {}.",
                            parts[0].to_string(),
                            parts[1].to_string(),
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
/// Выполняет команду.
///
/// Возвращает вывод команды.
///
fn run_command(mut command: InputValues) -> (String, ExitStatus) {
    let process = match Command::new(command.next().unwrap())
        .args(command)
        .output() {
        Ok(process) => process,
        Err(error) => panic!("Running process error: {}", error),
    };

    let output = String::from_utf8(process.stdout).unwrap();

    return (output, process.status)
}

///
/// Извлекает значения из вывода выполненной команды.
///
/// - output — вывод команды
/// - pattern — шаблон для поиска значений
///
fn extract_values(output: &String, pattern: Regex) -> HashMap<String, f32> {
    // Список найденных значений.
    let mut values = HashMap::new();

    for matches in pattern.captures_iter(output.as_str()) {
        for capture_name in pattern.capture_names() {
            let name = match capture_name {
                None => continue,
                Some(name) => name,
            };

            let one_match = match matches.name(name) {
                None => continue,
                Some(m) => m,
            };

            let value = match one_match.as_str().parse::<f32>() {
                Ok(value) => value,
                Err(error) => {
                    println!(
                        "Matched value \"{}\" for \"{}\" should be a float: {}.",
                        one_match.as_str(),
                        name,
                        error
                    );
                    exit(exitcode::DATAERR);
                }
            };

            let entry = values.entry(name.to_string()).or_insert(value);
            *entry = value;
        }
    }

    return values;
}

///
/// Проверяет значения на соответствие границам.
///
/// Возвращает список найденных проблем.
///
fn check_thresholds(values: HashMap<String, f32>, thresholds: Vec<Threshold>) -> Vec<String> {
    // Список найденных проблем.
    let mut failures: Vec<String> = vec![];

    for threshold in &thresholds {
        let value = match values.get(threshold.parameter.as_str()) {
            None => {
                println!("Notice: value for \"{}\" not found in command output.", threshold.parameter);
                continue;
            },
            Some(value) => *value
        };

        if !threshold.satisfied(value) {
            failures.push(
                format!(
                    "{} value {} is {} than {}",
                    threshold.parameter,
                    value,
                    match threshold.kind {
                        ThresholdKind::Lower => "lower",
                        ThresholdKind::Higher => "higher",
                    },
                    threshold.value
                )
            );
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

    let command = arguments.values_of("COMMAND").unwrap();
    let pattern = arguments.value_of("pattern").unwrap();
    let regex = Regex::new(pattern).unwrap();

    // Список пороговых значений.
    let mut thresholds: Vec<Threshold> = vec![];

    if arguments.is_present("lower") {
        for argument in arguments.values_of("lower") {
            thresholds.append(&mut parse_threshold_args(ThresholdKind::Lower, argument));
        }
    }

    if arguments.is_present("higher") {
        for argument in arguments.values_of("higher") {
            thresholds.append(&mut parse_threshold_args(ThresholdKind::Higher, argument));
        }
    }

    let ignore_status = arguments.is_present("ignore-status");

    let (command_output, command_status) = run_command(command);
    println!("{}", command_output.as_str());
    println!();

    let values = extract_values(&command_output, regex);
    println!("Pattern matches:");
    for (parameter, value) in &values {
        println!("‣ {}: {}", parameter, value);
    }
    println!();

    if arguments.is_present("write-env") {
        let mut data = String::new();
        for (parameter, value) in &values {
            data.push_str(format!("{}={}\n", parameter.to_uppercase(), value).as_str());
        }

        let filename = arguments.value_of("write-env").unwrap();
        let mut file = File::create(filename)
            .expect(format!("Unable to create file \"{}\".", filename).as_str());
        file.write_all(data.as_bytes())
            .expect(format!("Unable to write to \"{}\".", filename).as_str());
    }

    let failures = check_thresholds(values, thresholds);

    if failures.len() > 0 {
        println!("Threshold failures:");
        for failure in failures {
            println!("❌ {}", failure);
        }
        exit(exitcode::DATAERR);
    }

    if !ignore_status {
        exit(command_status.code().unwrap());
    }
}
