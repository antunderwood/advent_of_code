use std::fs::File;
use std::io::{self, BufRead};

fn find_jolt(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

    let max_val = digits[..(digits.len() - 1)]
        .iter()
        .max()
        .unwrap();

    let max_pos = digits
        .iter()
        .position(|v| v == max_val)
        .unwrap();

    let next_max_val = digits[max_pos + 1..]
        .iter()
        .max()
        .unwrap();
    
    max_val * 10 + next_max_val  
}
fn main() {
    // Read the data from the file
    let mut lines: Vec<String> = Vec::new();
    if let Ok(file) = File::open("data.txt") {
        let reader = io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            lines.push(line);
        }
    }
    let mut jolts: Vec<u32> = Vec::new();
    for line in lines {
        let jolt = find_jolt(&line);
        jolts.push(jolt);
    }
    let sum: u32 = jolts.iter().sum();
    println!("Jolt sum: {sum:?}, Length: {length:?}", length = jolts.len());
}

#[cfg(test)]
mod tests {
    use super::find_jolt;

    #[test]
    fn test_find_jolt_987654321111111() {
        // In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
        assert_eq!(find_jolt("987654321111111"), 98);
    }

    #[test]
    fn test_find_jolt_811111111111119() {
        // In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
        assert_eq!(find_jolt("811111111111119"), 89);
    }

    #[test]
    fn test_find_jolt_234234234234278() {
        // In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
        assert_eq!(find_jolt("234234234234278"), 78);
    }

    #[test]
    fn test_find_jolt_818181911112111() {
        // In 818181911112111, the largest joltage you can produce is 92.
        assert_eq!(find_jolt("818181911112111"), 92);
    }

    #[test]
    fn test_jolt_sum() {
        // sum should be 357
        let lines = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let jolts = lines.iter().map(|line| find_jolt(line)).collect::<Vec<u32>>();
        let sum: u32 = jolts.iter().sum();
        assert_eq!(sum, 357);
    }

}
