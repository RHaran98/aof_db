use std::sync::{Arc};
use std::thread;
use rand::{Rng, SeedableRng, seq::IteratorRandom};
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
mod db;
use db::StringDB;
use std::time::{ Instant};
// mod sync_db;
// use sync_db::StringDBSync;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_strings = 5000;
    let num_rem_strings = num_strings / 2;
    let string_length = 300;

    let mut _r = StdRng::seed_from_u64(222);

    // Generate random strings
    let strings: Vec<String> = (0..num_strings)
        .map(|_| generate_random_string(string_length))
        .collect();

    let mut rng = rand::thread_rng();
    let mut remove_strings = strings.clone();
    remove_strings.shuffle(&mut rng);
    remove_strings.truncate(num_rem_strings);

    // let remove_strings: Vec<String> = strings.iter().choose_multiple(&mut rng, num_rem_strings);

    let db = StringDB::new("test.sqlite").unwrap();
    db.flush();
    let add_strings = strings.clone();
    let rem_strings = remove_strings.clone();
    let start = Instant::now();
    add_strings_concurrently( add_strings, rem_strings);
    let duration = start.elapsed();
    println!("Async done in {:?}", duration);

    let add_strings = strings.clone();
    let rem_strings = remove_strings.clone();
    db.flush();
    let start = Instant::now();
    add_strings_concurrently( add_strings, rem_strings);
    let duration = start.elapsed();
    println!("Sync done in {:?}", duration);
    Ok(())
}


fn generate_random_string(length: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    string
}

fn add_strings_concurrently( strings: Vec<String>, rem_strings: Vec<String>) {
    let strings = Arc::new(strings);
    let num_threads = 4; // Number of threads to use (adjust as needed)

    let chunk_size = (strings.len() + num_threads - 1) / num_threads;
    let chunks: Vec<_> = strings.chunks(chunk_size).collect();

    let mut handles = Vec::new();

    for chunk in chunks {
        let chunk = chunk.to_owned();
        let db = StringDB::new("test.sqlite").unwrap();
        let handle = thread::spawn(move || {
            
            for string in chunk {
                db.add(&string);
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }


    let chunk_size = (rem_strings.len() + num_threads - 1) / num_threads;
    let chunks: Vec<_> = rem_strings.chunks(chunk_size).collect();

    let mut handles = Vec::new();

    for chunk in chunks {
        let chunk = chunk.to_owned();
        let db = StringDB::new("test.sqlite").unwrap();
        let handle = thread::spawn(move || {
            
            for string in chunk {
                db.remove(&string);
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn add_strings_sequentially( strings: Vec<String>, rem_strings: Vec<String>) {
    let db = StringDB::new("test.sqlite").unwrap();
    for string in &strings {
        db.add(&string);
    }

    for string in &rem_strings {
        db.remove(&string);
    }

}
