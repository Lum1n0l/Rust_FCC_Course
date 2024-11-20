// This program takes a string/MD5 Hash as input, computes its MD5 hash if its a string, and then attempts to brute-force the original string by generating all possible combinations of characters and comparing their MD5 hashes to the original hash.


// Import necessary modules from the standard library and the md5 crate
use std::io::{self, Write};
// use std::time::Instant;
use md5;

// Function to generate all possible combinations of characters
fn generate_combinations(chars: &[char], length: usize, current: &mut Vec<char>, results: &mut Vec<String>) {
    // If the current combination has reached the desired length, add it to the results
    if current.len() == length {
        results.push(current.iter().collect());
        return;
    }
    // Iterate through each character in the character set
    for &ch in chars {
        // Add the character to the current combination
        current.push(ch);
        // Recursively generate combinations with the next character
        generate_combinations(chars, length, current, results);
        // Remove the last character to backtrack and try the next character
        current.pop();
    }
}

fn main() {
    // Prompt the user to choose an option
    println!("Choose an option:");
    println!("1. Input a string to convert to MD5 hash");
    println!("2. Input an MD5 hash directly");
    print!("Enter your choice (1 or 2): ");
    io::stdout().flush().unwrap();

    // Read the user's choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    // Variable to store the MD5 hash
    let input_hash;

    if choice == "1" {
        // Option 1: Input a string to convert to MD5 hash
        print!("Enter a string: ");
        io::stdout().flush().unwrap();
        
        // Read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Convert the input to an MD5 hash
        input_hash = format!("{:x}", md5::compute(input));
    } else if choice == "2" {
        // Option 2: Input an MD5 hash directly
        print!("Enter an MD5 hash: ");
        io::stdout().flush().unwrap();
        
        // Read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input_hash = input.trim().to_string();
    } else {
        println!("Invalid choice.");
        return;
    }

    // Define the character set to use for brute-forcing
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();

    // Variable to track if the input has been successfully guessed
    let mut found = false;

    let start_time = std::time::Instant::now();

    // Iterate through possible lengths of the input
    for length in 1..=32 { // Assuming the input length is at most 32 characters
        // Vector to store all generated combinations of the current length
        let mut results = Vec::new();
        // Generate all combinations of the current length
        generate_combinations(&charset, length, &mut Vec::new(), &mut results);
        // Iterate through each generated combination
        for candidate in results {
            // Check if the MD5 hash of the candidate matches the input hash
            if format!("{:x}", md5::compute(&candidate)) == input_hash {
                // If a match is found, print the matching candidate
                let elapsed = start_time.elapsed();
                println!("Found match: {}", candidate);
                println!("Cracked in: {:?}", elapsed);

                // Set the found flag to true and break out of the loop
                found = true;
                break;
            }
        }
        // If a match was found, break out of the outer loop
        if found {
            break;
        }
    }

    // If no match was found, print a message indicating so
    if !found {
        println!("No match found.");
    }
}
