// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
// * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

#[derive(Debug)]
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(option: String) -> Option<PowerState> {
        match option.to_lowercase().as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }

    fn print(&self) {
        match self {
            PowerState::Off => println!("off"),
            PowerState::Sleep => println!("sleeping"),
            PowerState::Reboot => println!("rebotting"),
            PowerState::Shutdown => println!("shutting down"),
            PowerState::Hibernate => println!("hibernating"),
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.to_lowercase().trim().to_string())
}

fn main() {
    let input = get_input().unwrap();
    let option = PowerState::new(input.to_owned());

    match option {
        Some(val) => val.print(),
        None => println!("invalid option"),
    }
}
