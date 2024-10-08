use std::{env, process::exit};
use rand::Rng;

const ALPHABET: &str = "abcdefghijklmopqrstuvwxyz";
const SPECIALS: &str = "?!-_@#()[]{}><':/;.+=%*$€&";
const NUMBERS: &str = "0123456789";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct Parameters {
    length: u8,
    generator_string: String
}

fn is_integer(s: &str) -> bool {
    s.parse::<u8>().is_ok()
}

struct NumberHolder {
    number: Option<u8>,
}

impl NumberHolder {
    fn from_str(s: &str) -> Self {
        // Try to parse the string into a u8, then check if it's in the valid range
        let number = s.parse::<u8>().ok().filter(|&n| n >= 6 && n <= 30);
        NumberHolder { number }
    }
}

fn print_help() {
    println!("HELP MENU PASSWORD GENERATOR");
    println!("\n--length [nb_char (u8)]   Give the amount of character you need for your password (6 by default, can be up to 30 max)");
    println!("\n--uppercase   Include uppercase characters in the password");
    println!("\n--numbers     Include numbers in the password");
    println!("\n--specials    Include special characters in the password");
    println!("\n--help | -h   Display the help menu");
    println!("-----------------------------------------------------------------------------------------------------------------------");
    println!("Coded by Thibault \"Flash2Vovo\" BARBE");
    exit(0);
}

fn get_parameters(arguments: Vec<String>) -> Parameters {
    let mut new_params = Parameters {
        length: 6,
        generator_string: String::from(ALPHABET)
    };

    let mut i = 0;
    while i < arguments.len() {
        if arguments[i] == "--uppercase" {
            new_params.generator_string = new_params.generator_string + UPPERCASE;
        } else if arguments[i] == "--numbers" {
            new_params.generator_string = new_params.generator_string + NUMBERS;
        } else if arguments[i] == "--specials" {
            new_params.generator_string = new_params.generator_string + SPECIALS;
        } else if arguments[i] == "--length" {
            if is_integer(arguments[i + 1].as_str()) == true {
                let valid_holder = NumberHolder::from_str(arguments[i + 1].as_str());
                match valid_holder.number {
                    Some(num) => new_params.length = num,
                    None => println!("The password's length might stay between 6 and 30 character long"),
                };
            };
        } else if arguments[i] == "--help" || arguments[i] == "-h" {
            print_help();
        }
        i = i + 1;
    }

    new_params
}

fn generate_password(parameters: Parameters) {
    let mut password = String::new();

    for _n in 1..parameters.length {
        let rand_index = rand::thread_rng().gen_range(0..parameters.generator_string.len());
        password.push(parameters.generator_string.chars().nth(rand_index).unwrap());
    };

    println!("Password generated: {:?}", password.as_str());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parameters: Parameters = get_parameters(args);

    println!("Password generator!");

    generate_password(parameters);
}
