extern crate core;

mod file_entries;

use std::time::Instant;
use file_entries::types::ItemType;

fn main() {
    let now = Instant::now();

    let item_type = ItemType::process_path("C:\\Program Files");
    item_type.print(2, 0);
    // match item_type {
    //     ItemType::Directory { size, path, children } => {
    //         println!("total size of {path}: {size}")
    //     }
    //     ItemType::File { size, path, .. } => {
    //         println!("total size of {path}: {size}")
    //     }
    // }

    let elapsed = now.elapsed();
    println!("{:.2?}", elapsed);

}

