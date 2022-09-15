extern crate core;

mod file_entries;

use std::time::Instant;
use file_entries::types::ItemType;

fn main() {
    let now = Instant::now();

    let item_type = ItemType::process_path("C:\\Program Files");
    item_type.print(2, 0);
    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);

}

