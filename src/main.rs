#[macro_use]

extern crate clap;
extern crate exitcode;
extern crate regex;

use clap::App;
use regex::Regex;
use std::process::Command;
use std::process::exit;
use std::string::String;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let arguments = App::from_yaml(yaml).get_matches();

    let mut command = arguments.values_of("COMMAND").unwrap();
    let pattern = arguments.value_of("pattern").unwrap();
    let mut lowers = arguments.values_of("lowers").unwrap();
    let mut highers = arguments.values_of("highers").unwrap();

    let process = match Command::new(command.next().unwrap())
        .args(command)
        .output() {
        Ok(process) => process,
        Err(error) => panic!("Running process error: {}", error),
    };

    let output = String::from_utf8(process.stdout).unwrap();

    let mut exit_code = exitcode::OK;

    let re = Regex::new(pattern).unwrap();
    for matches in re.captures_iter(output.as_str()) {
        let lower_thresholds = &mut lowers;
        for threshold in lower_thresholds {
            let parts: Vec<&str> = threshold.split("=").collect();
            let actual_value = matches.name(parts[0]).unwrap().as_str();
            let threshold_value = parts[1];
            if actual_value < threshold_value {
                exit_code = exitcode::DATAERR;
            }
        }
        let higher_thresholds = &mut highers;
        for threshold in higher_thresholds {
            let parts: Vec<&str> = threshold.split("=").collect();
            let actual_value = matches.name(parts[0]).unwrap().as_str();
            let threshold_value = parts[1];
            if actual_value > threshold_value {
                exit_code = exitcode::DATAERR;
            }
        }
    }

    println!("{}", output.as_str());

    exit(exit_code);
}
