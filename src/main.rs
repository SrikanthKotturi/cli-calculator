use clap::{value_parser, Arg, ArgAction, Command};
mod calculator;

fn main() {
    let about = "Rust command-line calculator\nAuthor: Srikanth Kotturi";

    let matches = Command::new("cli-calc")
        .version("1.0")
        .author("Srikanth Kotturi")
        .about(about)
        .arg(
            Arg::new("about")
                .short('A')
                .long("about")
                .help("Show the about information")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("operation")
                .short('o')
                .long("operation")
                .value_name("OPERATION")
                .help("Arithmetic operation to perform")
                .required(false)
                .num_args(1)
                .value_parser(["add", "+", "sub", "-", "mul", "*", "div", "/"]),
        )
        .arg(
            Arg::new("operand1")
                .long("o1")
                .help("First operand")
                .required(false)
                .value_parser(value_parser!(f64)),
        )
        .arg(
            Arg::new("operand2")
                .long("o2")
                .help("Second operand")
                .required(false)
                .value_parser(value_parser!(f64)),
        )
        .get_matches();

    if matches.get_flag("about") {
        println!("{}", about);
        return;
    }

    let operation = match matches.get_one::<String>("operation") {
        Some(op) => op,
        None => {
            eprintln!("No operation specified. Exiting.");
            std::process::exit(1);
        }
    };

    let operand1 = match matches.get_one::<f64>("operand1") {
        Some(&op1) => op1,
        None => {
            eprintln!("First operand not specified. Exiting.");
            std::process::exit(1);
        }
    };

    let operand2 = match matches.get_one::<f64>("operand2") {
        Some(&op2) => op2,
        None => {
            eprintln!("Second operand not specified. Exiting gracefully.");
            std::process::exit(1);
        }
    };

    let result = match operation.as_str() {
        "add" | "+" => calculator::add(&[operand1, operand2]),
        "sub" | "-" => calculator::subtract(&[operand1, operand2]),
        "mul" | "*" => calculator::multiply(&[operand1, operand2]),
        "div" | "/" => calculator::divide(&[operand1, operand2]).unwrap_or(f64::NAN),
        _ => {
            eprintln!("Invalid operation: {}", operation);
            std::process::exit(1);
        }
    };

    println!("Result: {}", result);
}
