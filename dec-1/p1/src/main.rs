use std::path::Path;
use std::fs;

fn main() {

    // Read contents of txt file containing data
    let txt_path = Path::new("calibration.txt");
    let contents = fs::read_to_string(txt_path).expect("ERROR: Could not read file to string D:");
    
    // Split each line, and place all the new strings into a Vec![]
    let messy_calibrations: Vec<String> = contents.split("\n").map(|urmom| urmom.to_string()).collect();
    
    // Our answer
    let mut sum: u64 = 0;
    
    // Iterate thru vec of strings
    for label in messy_calibrations.iter() {

        let mut firstnum: Option<u64> = None;
        let mut lastnum: Option<u64> = None;
        
        // Iterate thru the string's characters
        for char in label.chars() {
            // Handle most cases; if the string contains 2 or more digits 
            if char.is_numeric() && firstnum == None {
                firstnum = Some(char.to_digit(10).unwrap() as u64);
            } else if char.is_numeric() && firstnum != None {
                lastnum = Some(char.to_digit(10).unwrap() as u64);
            }
        }
        // Handle case where string contains only 1 digit
        if lastnum == None {
            lastnum = firstnum;
        }

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
