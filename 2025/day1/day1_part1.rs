use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads the lines from "rotations.txt" into a Vec<String>
fn import_rotations() -> io::Result<Vec<String>> {
    let path = Path::new("rotations.txt");
    let file = File::open(path)?;
    io::BufReader::new(file).lines().collect()
}
/// Counts the number of times position 0 is passed or landed on given rotations and an initial position.
///
/// # Arguments
///
/// * `rotations` - Vector of (direction, steps) e.g. [('L', 50), ('R', 50)]
/// * `initial_position` - Starting position on the dial
///
/// # Returns
///
/// Number of times position 0 is passed or landed on.
fn count_zero_occurrences(rotations: Vec<(char, i32)>, initial_position: i32) -> i32 {
    let mut position = initial_position;
    let mut num_occurences_past_or_landing_on_zero = 0;

    for (direction, steps) in rotations {

        if direction == 'L' {
            let transformed_position = (100 - position) % 100;
            let number_passes_through_zero = (transformed_position + steps) / 100;
            num_occurences_past_or_landing_on_zero += number_passes_through_zero;
            position = (100 - ((transformed_position + steps) % 100)) % 100;
        } else if direction == 'R' {
            let number_passes_through_zero = (position + steps) / 100;
            num_occurences_past_or_landing_on_zero += number_passes_through_zero;
            position = (position + steps) % 100;
        }
    }
    num_occurences_past_or_landing_on_zero
}

fn main() {
    let position = 50;
    let rotations = import_rotations().unwrap();
    let mut parsed_rotations: Vec<(char, i32)> = Vec::new();
    for s in &rotations {
        if let Some(first_char) = s.chars().next() {
            let rest: String = s.chars().skip(1).collect();
            if let Ok(num) = rest.parse::<i32>() {
                parsed_rotations.push((first_char, num));
            }
        }
    }
    let num_occurences_zero = count_zero_occurrences(parsed_rotations, position);
    println!("Number of occurences of 0: {num_occurences_zero}");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l50_r50_equals_1() {
        // Starts at 50; L50,R50
        let rotations = vec![('L', 50), ('R', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 1);
    }

    #[test]
    fn test_l50_l50_equals_1() {
        // Starts at 50; L50,L50
        let rotations = vec![('L', 50), ('L', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 1);
    }

    #[test]
    fn test_r50_l50_equals_1() {
        // Starts at 50; R50,L50
        let rotations = vec![('R', 50), ('L', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 1);
    }

    #[test]
    fn test_r50_r50_equals_1() {
        // Starts at 50; R50,R50
        let rotations = vec![('R', 50), ('R', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 1);
    }

    #[test]
    fn test_l150_l50_equals_2() {
        // Starts at 50; L150,L50
        let rotations = vec![('L', 150), ('L', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 2);
    }

    #[test]
    fn test_l150_r50_equals_2() {
        // Starts at 50; L150,R50
        let rotations = vec![('L', 150), ('R', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 2);
    }

    #[test]
    fn test_r150_l50_equals_2() {
        // Starts at 50; R150,L50
        let rotations = vec![('R', 150), ('L', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 2);
    }

    #[test]
    fn test_r150_r50_equals_2() {
        // Starts at 50; R150,R50
        let rotations = vec![('R', 150), ('R', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 2);
    }

    #[test]
    fn test_r250_r50_equals_3() {
        // Starts at 50; R150,R50
        let rotations = vec![('R', 250), ('R', 50)];
        assert_eq!(count_zero_occurrences(rotations, 50), 3);
    }

    #[test]
    fn test_r250_l100_equals_4() {
        // Starts at 50; R150,L100
        let rotations = vec![('R', 250), ('L', 100)];
        assert_eq!(count_zero_occurrences(rotations, 50), 4);
    }
}

