use std::fs::File;
use std::io::{self, BufRead};

fn find_jolt(line: &str, length: usize) -> u64 {
    let digits: Vec<u32> = line.chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

    let mut start_pos = 0;
    let mut batteries: Vec<u32> = Vec::new();
    for i in (1..=length).rev() {
        let max_val = digits[start_pos..(digits.len() - i + 1)]
            .iter()
            .max()
            .copied()
            .unwrap();

        batteries.push(max_val);
        start_pos = digits
            .iter()
            .copied()
            .enumerate()
            .position(|(idx, v)| v == max_val && idx >= start_pos)
            .unwrap() + 1;
    }

    let jolt = batteries.iter().fold(0, |acc, x| acc * 10 + *x as u64);
    jolt
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
    let mut jolts: Vec<u64> = Vec::new();
    for line in lines {
        let jolt = find_jolt(&line, 12);
        jolts.push(jolt);
    }
    let sum: u64 = jolts.iter().sum();
    println!("Jolt sum: {sum:?}, Length: {length:?}", length = jolts.len());
}

#[cfg(test)]
mod tests {
    use super::find_jolt;

    #[test]
    fn test_find_jolt_987654321111111() {
        // In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
        assert_eq!(find_jolt("987654321111111", 12), 987654321111);
    }

    #[test]
    fn test_find_jolt_811111111111119() {
        // In 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
        assert_eq!(find_jolt("811111111111119", 12), 811111111119);
    }

    #[test]
    fn test_find_jolt_234234234234278() {
        // In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
        assert_eq!(find_jolt("234234234234278", 12), 434234234278);
    }

    #[test]
    fn test_find_jolt_818181911112111() {
        // In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.
        assert_eq!(find_jolt("818181911112111", 12), 888911112111);
    }

    #[test]
    fn test_jolt_sum() {
        // sum should be 3121910778619
        let lines = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let jolts = lines.iter().map(|line| find_jolt(line, 12)).collect::<Vec<u64>>();
        let sum: u64 = jolts.iter().sum();
        assert_eq!(sum, 3121910778619);
    }
}

