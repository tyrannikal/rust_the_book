use std::fmt;
use std::io;
use std::mem;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_RATIO: f64 = 1.8;
const KELVIN_OFFSET: f64 = 273.15;
const RANKINE_OFFSET: f64 = 491.67;

#[derive(Debug)]
enum ScaleType {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
}

impl ScaleType {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "c" => Some(ScaleType::Celsius),
            "f" => Some(ScaleType::Fahrenheit),
            "k" => Some(ScaleType::Kelvin),
            "r" => Some(ScaleType::Rankine),
            _ => None,
        }
    }
}

impl fmt::Display for ScaleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ScaleType::Celsius => "Celsius",
            ScaleType::Fahrenheit => "Fahrenheit",
            ScaleType::Kelvin => "Kelvin",
            ScaleType::Rankine => "Rankine",
        };
        write!(f, "{}", name)
    }
}

fn main() {
    println!("Convert temperatures between Celsius, Fahrenheit, Kelvin, and Rankine. Here is how it works:\n
        Enter the temperature: like 72 or -481.935\n
        Scale of the temperature: (c/f/k/r)\n
        Convert to: (c/f/k/r)\n
        ------------------------- Let's Go -------------------------"
    );

    'convert: loop {
        let temperature: f64 = loop {
            println!("Temperature:");

            match get_input().trim().parse() {
                Ok(num) => {
                    break num;
                }
                Err(_) => {
                    println!(
                        "Temperature must be a number, positive or negative, with or without a decimal."
                    );
                    continue;
                }
            }
        };

        let scale = get_scale_input("Scale (c/f/k/r):");

        let new_scale = get_scale_input("Convert to (c/f/k/r):");

        let new_temperature = convert(temperature, &scale, &new_scale);

        println!("{temperature} degrees {scale} is {new_temperature} degrees {new_scale}");

        loop {
            println!("Convert another temperature? (y/n):");

            let convert_again = get_input().trim().to_lowercase();

            if convert_again == "y" {
                break;
            }
            if convert_again == "n" {
                break 'convert;
            }
            println!("Input 'y' or 'n' to continue");
        }
    }

    println!("Goodbye!");
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn get_scale_input(prompt: &str) -> ScaleType {
    loop {
        println!("{}", prompt);
        let input = get_input().trim().to_lowercase();
        match ScaleType::from_str(&input) {
            Some(scale_type) => return scale_type,
            None => println!("Must be 'c', 'f', 'k', or 'r'"),
        }
    }
}

fn convert(temperature: f64, from: &ScaleType, to: &ScaleType) -> f64 {
    if mem::discriminant(from) == mem::discriminant(to) {
        return temperature;
    }

    let celsius = match from {
        ScaleType::Celsius => temperature,
        ScaleType::Fahrenheit => (temperature - FAHRENHEIT_OFFSET) / FAHRENHEIT_RATIO,
        ScaleType::Kelvin => temperature - KELVIN_OFFSET,
        ScaleType::Rankine => (temperature - RANKINE_OFFSET) / FAHRENHEIT_RATIO,
    };

    match to {
        ScaleType::Celsius => celsius,
        ScaleType::Fahrenheit => (celsius * FAHRENHEIT_RATIO) + FAHRENHEIT_OFFSET,
        ScaleType::Kelvin => celsius + KELVIN_OFFSET,
        ScaleType::Rankine => (celsius + KELVIN_OFFSET) * FAHRENHEIT_RATIO,
    }
}
