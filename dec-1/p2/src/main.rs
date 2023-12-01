use std::path::Path;
use std::collections::HashMap;
use std::fs;

fn main() {

    // Read contents of txt file containing data
    let txt_path = Path::new("calibration.txt");
    let contents = fs::read_to_string(txt_path).expect("ERROR: Could not read file to string D:");
    
    // Split each line, and place all the new strings into a Vec![]
    let messy_calibrations: Vec<String> = contents.split("\n").map(|urmom| urmom.to_string()).collect();

    let mut sum: u64 = 0;

    // Create a mapping from number words to digits
    let word_to_num: HashMap<&str, u64> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();

    // Iterate thru vec of strings
    for label in messy_calibrations.iter() {

        let mut firstnum: Option<u64> = None;
        let mut lastnum: Option<u64> = None;
        
        
        // Iterate thru the string's characters
        for (i, _) in label.chars().enumerate() {
            for (word, &num) in word_to_num.iter() {
                if label[i..].starts_with(word) {
                    if firstnum.is_none() {
                        firstnum = Some(num);
                    }
                    lastnum = Some(num); // Always update lastnum
                }
            }
            // Check if the character is a digit
            let char_num = label[i..i+1].parse::<u64>();
            if let Ok(num) = char_num {
                if firstnum.is_none() {
                    firstnum = Some(num);
                }
                lastnum = Some(num); // Always update lastnum
            }
        }

        // Handle case where string contains only 1 digit
        if lastnum == None {
            lastnum = firstnum;
        }
        // Handle case where the string contains 0 digits
        let num_to_sum = if firstnum == None && lastnum == None {
            0
        } else {
            let tens = (firstnum.unwrap() * 10) as u64;
            let ones = lastnum.unwrap() as u64;
            tens + ones
        };
        sum += num_to_sum;

    }
    println!("{}", sum);
}


