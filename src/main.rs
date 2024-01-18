use std::fs::{self, OpenOptions};
use std::io::BufWriter;
use std::io::Write;

const LARGE_FILE_SIZE: usize = 2_097_152; // 2GB in KB
const MEDIUM_FILE_SIZE: usize = 204_800; // 200MB in KB
const SMALL_FILE_SIZE: usize = 24; // 24KB in KB
const LARGE_FILE_COUNT: usize = 10;
const MEDIUM_FILE_COUNT: usize = 100;
const SMALL_FILE_COUNT: usize = 999_890;

fn main() {
    let drive_location = "/Volumes/Encrypted";

    // Create root directories
    fs::create_dir_all(format!("{}/src", drive_location)).unwrap();
    fs::create_dir_all(format!("{}/dst", drive_location)).unwrap();

    println!("Root directories created.");

    // Generate large files
    for i in 0..LARGE_FILE_COUNT {
        let file_path = format!("{}/src/large_file{}.txt", drive_location, i);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let mut writer = BufWriter::new(file);
        for _ in 0..LARGE_FILE_SIZE {
            writer.write_all(&[0; 1024]).unwrap(); // Write 1KB of zeros
        }
        println!("Large file {} created.", i);
    }

    // Generate medium files
    for i in 0..MEDIUM_FILE_COUNT {
        let file_path = format!("{}/dst/medium_file{}.txt", drive_location, i);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let mut writer = BufWriter::new(file);
        for _ in 0..MEDIUM_FILE_SIZE {
            writer.write_all(&[0; 1024]).unwrap(); // Write 1KB of zeros
        }
        println!("Medium file {} created.", i);
    }

    // Generate small files
    for i in 0..SMALL_FILE_COUNT {
        let dir = if i % 2 == 0 { "src" } else { "dst" };
        let file_path = format!("{}/{}/small_file{}.txt", drive_location, dir, i);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let mut writer = BufWriter::new(file);
        for _ in 0..SMALL_FILE_SIZE {
            writer.write_all(&[0; 1024]).unwrap(); // Write 1KB of zeros
        }
        if i % 10000 == 0 {
            // Print progress every 10000 files
            println!("Small file {} created.", i);
        }
    }

    println!("All files created.");
}
