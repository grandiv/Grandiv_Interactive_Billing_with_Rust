mod bill;
mod bill_manager;
mod menu;

use bill_manager::BillManager;
use menu::run_menu;

fn main() {
    let mut bill_manager = BillManager::new();
    run_menu(&mut bill_manager);
}