use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

fn main() {
    let mut bills: HashMap<usize, Bill> = HashMap::new();
    let mut bill_count: usize = 0;

    loop {
        print_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: usize = input.trim().parse().unwrap_or(0);

        match choice {
            1 => add_bill(&mut bills, &mut bill_count),
            2 => view_bills(&bills),
            3 => remove_bill(&mut bills),
            4 => update_bill(&mut bills),
            5 => print_total(&bills),
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

fn add_bill(bills: &mut HashMap<usize, Bill>, bill_count: &mut usize) {
    print!("Enter bill name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");

    print!("Enter bill amount: ");
    io::stdout().flush().unwrap();
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read amount");

    let bill = Bill {
        name: name.trim().to_string(),
        amount: amount.trim().parse().unwrap_or(0.0),
    };

    bills.insert(*bill_count, bill);
    *bill_count += 1;
    println!("Bill added successfully.");
}

fn view_bills(bills: &HashMap<usize, Bill>) {
    if bills.is_empty() {
        println!("No bills found.");
        return;
    }

    println!("Bills:");
    for (id, bill) in bills {
        println!("{}: {} - Rp{:.2}", id, bill.name, bill.amount);
    }
}

fn remove_bill(bills: &mut HashMap<usize, Bill>) {
    print!("Enter bill ID to remove: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read ID");

    let id: usize = id.trim().parse().unwrap_or(0);

    if bills.contains_key(&id) {
        bills.remove(&id);
        println!("Bill removed successfully.");
    } else {
        println!("Invalid bill ID.");
    }
}

fn update_bill(bills: &mut HashMap<usize, Bill>) {
    print!("Enter bill ID to update: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read ID");

    let id: usize = id.trim().parse().unwrap_or(0);

    if let Some(bill) = bills.get_mut(&id) {
        print!("Enter new bill name (leave blank to keep current): ");
        io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");

        if !name.trim().is_empty() {
            bill.name = name.trim().to_string();
        }

        print!("Enter new bill amount (leave blank to keep current): ");
        io::stdout().flush().unwrap();
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Failed to read amount");

        if !amount.trim().is_empty() {
            bill.amount = amount.trim().parse().unwrap_or(bill.amount);
        }

        println!("Bill updated successfully.");
    } else {
        println!("Invalid bill ID.");
    }
}

fn print_total(bills: &HashMap<usize, Bill>) {
    let total: f64 = bills.values().map(|bill| bill.amount).sum();
    println!("Total bill amount: Rp{:.2}", total);
}