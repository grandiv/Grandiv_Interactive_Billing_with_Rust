use std::io::{self, Write};

use crate::bill_manager::BillManager;

pub fn run_menu(bill_manager: &mut BillManager) {
    loop {
        print_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: usize = input.trim().parse().unwrap_or(0);

        match choice {
            1 => add_bill(bill_manager),
            2 => bill_manager.view_bills(),
            3 => remove_bill(bill_manager),
            4 => update_bill(bill_manager),
            5 => bill_manager.print_total(),
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn print_menu() {
    println!("\n== Manage Bills ==");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Update bill");
    println!("5. Bill total");
    print!("Enter selection: ");
    io::stdout().flush().unwrap();
}

fn add_bill(bill_manager: &mut BillManager) {
    print!("Enter bill name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");

    print!("Enter bill amount: ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read amount");

    let name = name.trim().to_string();
    let amount = amount.trim().parse().unwrap_or(0.0);

    bill_manager.add_bill(name, amount);
}

fn remove_bill(bill_manager: &mut BillManager) {
    print!("Enter bill ID to remove: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read ID");

    let id: usize = id.trim().parse().unwrap_or(0);

    bill_manager.remove_bill(id);
}

fn update_bill(bill_manager: &mut BillManager) {
    print!("Enter bill ID to update: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read ID");

    let id: usize = id.trim().parse().unwrap_or(0);

    print!("Enter new bill name (leave blank to keep current): ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");

    let name = if name.trim().is_empty() {
        None
    } else {
        Some(name.trim().to_string())
    };

    print!("Enter new bill amount (leave blank to keep current): ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read amount");

    let amount = if amount.trim().is_empty() {
        None
    } else {
        Some(amount.trim().parse().unwrap_or(0.0))
    };

    bill_manager.update_bill(id, name, amount);
}