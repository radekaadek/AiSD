// Generate tests for main.rs in the tests directory

use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;
use std::thread::{self, JoinHandle};
use rand::rngs::ThreadRng;

// A program that generates tests for main.rs
// The tests are generated in the tests directory in the form of test_1.in, test_2.in, etc.
// Example usage: cargo run --bin test_gen -- 100
// This will generate n tests in the tests directory


// Function that generates test input strings
fn generate_test_input() -> String {
    // Create a random number generator
    let mut rng: ThreadRng = rand::thread_rng();

    // Generate a random number of scores and write it to the file
    let n: usize = rng.gen_range(1..100000);
    let mut input: String = format!("{}\n", n);

    // Generate a random number of score in a normal distribution and write it to the file
    for _ in 0..n {
        let score: u32 = rng.gen_range(1..101);
        input.push_str(format!("{} ", score).as_str());
    }
    input.push_str("\n");

    input
}

// Function that creates a test file
fn create_test_file(i: usize) -> File {
    // Check if the test file exists, if not, create it
    if std::path::Path::new(format!("tests/test_{}.in", i).as_str()).exists() {
        std::fs::remove_file(format!("tests/test_{}.in", i).as_str()).unwrap();
    }
    
    // Open the file
    let file: File = File::create(format!("tests/test_{}.in", i).as_str()).unwrap();

    file
}

// Function that creates a directory and test files
fn create_test_files(num_tests: usize) {
    // Create a vector of threads
    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(num_tests);

    // Check if the tests directory exists, if not, create it
    if !std::path::Path::new("tests").exists() {
        std::fs::create_dir("tests").unwrap_or_else(|_err|{
            exit_with_error("Failed to create tests directory\n");
        });
    }
    
    // Create a thread for each test
    for i in 1..num_tests+1 {
        let thread: JoinHandle<()> = thread::spawn(move || {
            // Create the test file
            let mut file: File = create_test_file(i);

            // Generate the test input
            let input: String = generate_test_input();

            // Write the test input to the file
            file.write_all(input.as_bytes()).unwrap();

            // Close the file
            file.sync_all().unwrap();

        });
        threads.push(thread);
    }

    // Join all threads
    for thread in threads {
        thread.join().unwrap();
    }

}

// Function that exits the program with an error message
fn exit_with_error(msg: &str) -> ! {
    print!("{}\n", msg);
    std::process::exit(1);
}

fn main() {
    // Get number of tests to generate from command line
    let args: Vec<String> = std::env::args().collect();

    // Assert that there is at least one argument
    if args.len() < 2 {
        exit_with_error("No number of tests specified");
    }

    // Parse the number of tests and convert it to a usize, if it fails,
    // tell the user by printing the error and exit the program
    let num_tests: usize = args[1].parse().unwrap_or_else( |_err: ParseIntError|{
        exit_with_error(format!("{} is not a valid number", args[1]).as_str());
    });

    // Assert that number of tests is in range 1..100
    if num_tests == 0 || num_tests > 100 {
        exit_with_error("Number of tests is not in range 1..100");
    }

    print!("Generating {} tests...\n", num_tests);
    create_test_files(num_tests);
    print!("Done!\n");

}