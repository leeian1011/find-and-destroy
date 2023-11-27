mod error;
use std::fs::{DirEntry, FileType};

use error::ShotError;

fn main() -> Result<(), ShotError> {
    let home = std::env::var("HOME").expect("Unable to find $HOME shell variable");

    _ = visit_and_remove(&home, "test.txt");

    Ok(())
}

fn visit_and_remove(dir: &str, to_remove: &str) -> Result<bool, ShotError> {
    // println!("Visiting dir: {}", dir);
    let directories = std::fs::read_dir(dir)?.collect::<Vec<_>>();
    for directory in directories {
        match directory {
            Err(dir_err) => {
                eprintln!("shot-away: failed to retrieve directories: {}", dir_err);
                return Ok(false);
            }
            Ok(dir_entry) => {
                if let Ok(dir_type) = dir_entry.file_type() {
                    if dir_type.is_dir() {
                        _ = visit_and_remove(
                            dir_entry.path().to_str().expect("Could not get path"),
                            to_remove,
                        );
                    } else if dir_type.is_file() {
                        // if dir_entry.file_name().to_str() == Some(to_remo) {
                        let mut name = dir_entry
                            .file_name()
                            .to_str()
                            .expect("Could not get file name.")
                            .chars()
                            .collect::<Vec<_>>();

                        name.reverse();
                        if name.len() < 4 {
                            continue;
                        }
                        if &name[0] == &'g'
                            && &name[1] == &'n'
                            && &name[2] == &'p'
                            && &name[3] == &'.'
                        {
                            println!("{:?}", name);
                        }

                        // match std::fs::remove_file(
                        //     dir_entry.path().to_str().expect("Could not get path"),
                        // ) {
                        //     Ok(_) => {}
                        //     Err(e) => eprintln!("{}", e),
                        // }
                        // }
                    }
                }
            }
        }
    }

    Ok(false)
}
