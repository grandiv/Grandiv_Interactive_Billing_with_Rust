use std::collections::HashMap;

use crate::bill::Bill;

pub struct BillManager {
    bills: HashMap<usize, Bill>,
    bill_count: usize,
}

impl BillManager {
    pub fn new() -> Self {
        BillManager {
            bills: HashMap::new(),
            bill_count: 0,
        }
    }

    pub fn add_bill(&mut self, name: String, amount: f64) {
        let bill = Bill {
            name,
            amount,
        };
        self.bills.insert(self.bill_count, bill);
        self.bill_count += 1;
        println!("Bill added successfully.");
    }

    pub fn view_bills(&self) {
        if self.bills.is_empty() {
            println!("No bills found.");
            return;
        }

        println!("Bills:");
        for (id, bill) in &self.bills {
            println!("{}: {} - Rp{:.2}", id, bill.name, bill.amount);
        }
    }

    pub fn remove_bill(&mut self, id: usize) {
        if self.bills.remove(&id).is_some() {
            println!("Bill removed successfully.");
        } else {
            println!("Invalid bill ID.");
        }
    }

    pub fn update_bill(&mut self, id: usize, name: Option<String>, amount: Option<f64>) {
        if let Some(bill) = self.bills.get_mut(&id) {
            if let Some(new_name) = name {
                bill.name = new_name;
            }
            if let Some(new_amount) = amount {
                bill.amount = new_amount;
            }
            println!("Bill updated successfully.");
        } else {
            println!("Invalid bill ID.");
        }
    }

    pub fn print_total(&self) {
        let total: f64 = self.bills.values().map(|bill| bill.amount).sum();
        println!("Total bill amount: Rp{:.2}", total);
    }
}