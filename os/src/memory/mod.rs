pub mod address;
pub mod config;
pub mod heap;

pub fn init() {
    heap::init();
    println!("mod memory initialized.")
}