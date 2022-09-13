// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;

struct Bill {
    description: String,
    amount: f64,
}

impl Bill {
    fn print(&self) {
        println!(
            "- Description: {:?} - Amount: {:?}",
            self.description, self.amount
        );
    }
}

pub struct Bills {
    items: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        let items = HashMap::new();
        Self { items }
    }

    fn add(&mut self, description: String, amount: f64) {
        self.items.insert(
            description.to_string(),
            Bill {
                description,
                amount,
            },
        );
    }

    fn set(
        &mut self,
        original_description: &String,
        description: String,
        amount: f64,
    ) -> Option<&mut Bill> {
        if let Some(bill) = self.items.get_mut(original_description) {
            bill.amount = amount;
            bill.description = description;
            Some(bill)
        } else {
            None
        }
    }

    fn remove(&mut self, description: &String) -> Option<Bill> {
        self.items.remove(description)
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.items.values().collect()
    }

    fn total_sum(&self) -> Option<f64> {
        self.get_all()
            .iter()
            .map(|bill| bill.amount)
            .collect::<Vec<f64>>()
            .into_iter()
            .reduce(|a, b| a + b)
    }
}

pub mod menu {
    use crate::*;
    use std::io;

    #[derive(Debug)]
    pub enum MenuItem {
        Add,
        View,
        Remove,
        Update,
        GetTotal,
    }

    impl MenuItem {
        pub fn from_str(value: &str) -> Option<MenuItem> {
            match value {
                "1" => Some(MenuItem::Add),
                "2" => Some(MenuItem::View),
                "3" => Some(MenuItem::Remove),
                "4" => Some(MenuItem::Update),
                "5" => Some(MenuItem::GetTotal),
                _ => None,
            }
        }
    }

    pub fn print() {
        println!("\n== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Bill total");
        println!("\nEnter selection:");
    }

    pub fn get_input() -> Option<String> {
        let mut buffer = String::new();
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("invalid input");
            return None;
        }
        Some(buffer.trim().to_owned())
    }

    pub fn parse_amount(val: &str) -> Option<f64> {
        match val.parse::<f64>() {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    pub fn add_bill(bills: &mut Bills) {
        println!("\n== Add Bill ==");
        println!("Description: ");
        let description = match get_input() {
            Some(val) => val,
            None => return,
        };
        println!("Amount: ");
        let amount_str = match get_input() {
            Some(val) => val.to_string(),
            None => return,
        };
        let amount = match parse_amount(&amount_str) {
            Some(val) => val,
            None => {
                println!("invalid amount");
                return;
            }
        };
        bills.add(description, amount);
        println!("Bill added successfully!");
    }

    pub fn view_bills(bills: &Bills) {
        let items = bills.get_all();
        println!("\n== Bills ({:?}) ==", items.len());
        for item in items {
            item.print();
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        println!("\n== Remove bill ==");

        for item in bills.get_all() {
            item.print();
        }

        println!("\nEnter the bill description you want to remove: ");

        let description = match get_input() {
            Some(val) => val,
            None => return,
        };

        match bills.remove(&description) {
            Some(_) => println!("Bill removed!"),
            None => println!("Bill not found"),
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        println!("\n== Update bill ==");

        for item in bills.get_all() {
            item.print();
        }

        println!("\nEnter the bill description you want to update: ");
        let original_description = match get_input() {
            Some(val) => val,
            None => return,
        };

        println!("\nEnter the new bill description: ");
        let description = match get_input() {
            Some(val) => val,
            None => return,
        };

        println!("\nEnter the new bill amount: ");
        let amount_str = match get_input() {
            Some(val) => val,
            None => return,
        };
        let amount = match parse_amount(&amount_str) {
            Some(val) => val,
            None => {
                println!("invalid amount");
                return;
            }
        };

        match bills.set(&original_description, description, amount) {
            Some(_) => println!("Bill updated!"),
            None => println!("Bill not found"),
        }
    }

    pub fn bill_total(bills: &Bills) {
        println!("\n== Bill total ==");
        match bills.total_sum() {
            Some(total) => println!("Total bill is: {:?}", total),
            None => println!("Could not sum all bills"),
        }
    }
}

fn main() {
    use menu::MenuItem;

    let mut bills = Bills::new();

    loop {
        menu::print();
        let input = menu::get_input();
        if input.is_none() {
            continue;
        }
        match MenuItem::from_str(&input.unwrap()) {
            Some(MenuItem::Add) => menu::add_bill(&mut bills),
            Some(MenuItem::View) => menu::view_bills(&bills),
            Some(MenuItem::Remove) => menu::remove_bill(&mut bills),
            Some(MenuItem::Update) => menu::update_bill(&mut bills),
            Some(MenuItem::GetTotal) => menu::bill_total(&bills),
            _ => {
                println!("invalid option");
                break;
            }
        }
    }
}
