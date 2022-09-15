use std::collections::{HashMap, VecDeque};
use std::fs::{Metadata, read};
use std::path::Path;
use humansize::{format_size, DECIMAL};

pub enum ItemType {
    Directory { children: Vec<ItemType>, size: u128, path: String },
    File { size: u64, path: String },
    NoAccess { path: String },
}

impl ItemType {
    pub fn add_child(&mut self, item: ItemType) {
        match self {
            ItemType::Directory { children, .. } => {
                children.push(item);
            }
            ItemType::File { .. } => {}
            ItemType::NoAccess { .. } => {}
        }
    }

    pub fn print(&self, recurse_limit: u8, recurse_level: u8) {
        if recurse_level >= recurse_limit {
            return;
        }

        match self {
            ItemType::Directory { size, path, children } => {
                let total_children = self.get_child_count();
                let size_output = format_size(*size as u64, DECIMAL);
                println!("total size of {path}: {size_output} ({total_children} children)");
                for c in children {
                    c.print(recurse_limit, recurse_level + 1);
                }
            }
            _ => {
            }
        }
    }

    pub fn get_child_count(&self) -> u32 {
        match self {
            ItemType::Directory { children, .. } => {
                let mut total = children.len() as u32;
                for c in children {
                    total += c.get_child_count();
                }
                total
            }
            _ => 0,
        }
    }

    // pub fn walk_path(root_path: &str) -> ItemType {
    //     let root = Path::new(root_path);
    //     let metadata = match root.metadata() {
    //         Ok(m) => m,
    //         Err(_) => {
    //             return ItemType::NoAccess {
    //                 path: String::from(root_path)
    //             }
    //         }
    //     };
    //
    //     let file_type = metadata.file_type();
    //
    //     if !file_type.is_dir() {
    //         return ItemType::File {
    //             size: metadata.len(),
    //             path: String::from(root_path)
    //         };
    //     }
    //
    //     let mut directories_to_walk: Vec<(&Path, Option<&mut ItemType>)> = Vec::new();
    //     directories_to_walk.push((root, None));
    //     let mut map_of_items: HashMap<String, &ItemType> = HashMap::new();
    //
    //     while !directories_to_walk.is_empty() {
    //         let pair = directories_to_walk.pop();
    //         if let Some(pair) = pair {
    //             let (current, parent) = pair;
    //
    //             let current_meta = current.metadata().expect("could not get current item metadata");
    //             if current_meta.is_file() {
    //                 Self::add_file_as_child(current.to_str().expect("current file name was missing"), parent, &current_meta)
    //             } else if current_meta.is_dir() {
    //                 let mut current_entry = ItemType::Directory {
    //                     path: String::from(current.to_str().expect("path does not have a path?")),
    //                     children: Vec::new(),
    //                     size: 0
    //                 };
    //
    //
    //                 let mut dir_size = 0;
    //                 let read_objects = current.read_dir();
    //                 if let Ok(read) = read_objects {
    //                     for obj in read {
    //                         if let Ok(obj) = obj {
    //                             let meta = obj.metadata().expect("failed to get metadata");
    //
    //                             if meta.is_file() {
    //                                 Self::add_file_as_child(obj.path().to_str().expect("could not convert to a path"), Some(&mut current_entry), &meta);
    //                             } else {
    //                                 directories_to_walk.push((parent_path, Some(&mut current_entry)));
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //
    //         }
    //     }
    //
    //     ItemType::NoAccess {
    //         path: String::new()
    //     }
    // }
    //
    // fn add_file_as_child(current_path: &str, parent: Option<&mut ItemType>, current_meta: &Metadata) {
    //     match parent {
    //         None => {
    //             panic!("No parent set but current entity is a file");
    //         }
    //         Some(&mut mut p) => {
    //             p.add_child(ItemType::File {
    //                 size: current_meta.len(),
    //                 path: String::from(current_path)
    //             })
    //         }
    //     }
    // }

    pub fn process_path(root_path: &str) -> ItemType {
        let root = Path::new(root_path);

        let metadata = match root.metadata() {
            Ok(m) => {
                m
            }
            Err(_) => {
                return ItemType::NoAccess {
                    path: String::from(root_path)
                }
            }
        };

        let file_type = metadata.file_type();

        if file_type.is_file() {
            return ItemType::File {
                size: metadata.len(),
                path: String::from(root_path)
            };
        } else if file_type.is_dir() {
            let mut children: Vec<ItemType> = Vec::new();
            let mut accumulated_size: u128 = 0;

            let sub_items = root.read_dir().expect("cannot read directory");
            for si in sub_items {
                let sub_item = si.expect("could not read sub item");
                let child = Self::process_path(sub_item.path().to_str().expect("Path shouldn't be empty"));
                match child {
                    ItemType::Directory { size, .. } => { accumulated_size += size },
                    ItemType::File { size, .. } => { accumulated_size += size as u128 },
                    ItemType::NoAccess {..} => {}
                }
                children.push(child);
            }

            return ItemType::Directory {
                children,
                size: accumulated_size,
                path: String::from(root_path),
            };
        } else {
            panic!("Unhandled item for {:?}", root);
        }
    }
}