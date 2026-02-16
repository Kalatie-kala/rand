//This program is a basic bruteforce tester/Complexity analysis thingy.
//RUNTIME IS NOT THE BEST and no gpu support in this version

use std::time::Instant;

fn main() {
    //Change this to the data set for actual usage
    let target_password = "passwordshere";
    
    println!("Target password: {}", target_password);
    
    let start = Instant::now();

    if let Some(found) = brute_force_simple(target_password, 3) {
        let duration = start.elapsed();
        println!("\n✓ Password found: {}", found);
        println!("Time taken: {:?}", duration);
    } else {
        println!("\n✗ Password not found");
    }
    println!("\n=== Complexity Analysis ===");
    demo_complexity();
}
fn brute_force_simple(target: &str, max_length: usize) -> Option<String> {
    let charset: String = (32..=126).map(|c| c as u8 as char).collect();
    // " !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
    
    let mut attempts = 0;
    for length in 1..=max_length {
        if let Some(password) = try_length(target, &charset, length, &mut attempts) {
            return Some(password);
        }
    }
    None
}
fn try_length(target: &str, charset: &str, length: usize, attempts: &mut u64) -> Option<String> {
    let chars: Vec<char> = charset.chars().collect();
    let mut current = vec![0; length];
    loop {
        let attempt: String = current.iter().map(|&i| chars[i]).collect();
        *attempts += 1;
        if *attempts % 1000 == 0 {
            print!("\rAttempts: {} | Current: {}", attempts, attempt);
        }
        if attempt == target {
            return Some(attempt);
        }
        let mut pos = length - 1;
        loop {
            current[pos] += 1;
            if current[pos] < chars.len() {
                break;
            }
            current[pos] = 0;
            if pos == 0 {
                return None;
            }
            pos -= 1;
        }
    }
}

fn demo_complexity() {
    let charset_size = 26;
    
    println!("For lowercase letters only (26 characters):");
    for length in 1..=8 {
        let combinations: u64 = charset_size.pow(length);
        println!("  Length {}: {:>15} possible combinations", length, format_number(combinations));

        let seconds = combinations as f64 / 1_000_000.0;
        if seconds < 60.0 {
            println!("             ~{:.2} seconds at 1M attempts/sec", seconds);
        } else if seconds < 3600.0 {
            println!("             ~{:.2} minutes at 1M attempts/sec", seconds / 60.0);
        } else if seconds < 86400.0 {
            println!("             ~{:.2} hours at 1M attempts/sec", seconds / 3600.0);
        } else if seconds < 31536000.0 {
            println!("             ~{:.2} days at 1M attempts/sec", seconds / 86400.0);
        } else {
            println!("             ~{:.2} years at 1M attempts/sec", seconds / 31536000.0);
        }
    }
    
    println!("\nWith uppercase + lowercase + digits (62 chars):");
    let charset_size = 62;
    for length in 6..=10 {
        let combinations: u64 = charset_size.pow(length);
        let seconds = combinations as f64 / 1_000_000.0;
        println!("  Length {}: {:>18} combinations (~{:.2} years)", 
                 length, 
                 format_number(combinations),
                 seconds / 31536000.0);
    }
}

fn format_number(n: u64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}
