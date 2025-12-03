fn find_invalid_ids(start: i64, end: i64) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    for num in start..=end {
        if num >= 10 && num.to_string().len() % 2 == 0 {
            let num_str = num.to_string();
            let mid = num_str.len() / 2;
            let first_num = num_str[..mid].parse::<i64>().unwrap();
            let second_num = num_str[mid..].parse::<i64>().unwrap();
            if first_num == second_num {
                invalid_ids.push(num);
            }
        }
    }
    invalid_ids
}

fn main() {
    let ranges_string = "18623-26004,226779-293422,65855-88510,868-1423,248115026-248337139,903911-926580,97-121,67636417-67796062,24-47,6968-10197,193-242,3769-5052,5140337-5233474,2894097247-2894150301,979582-1016336,502-646,9132195-9191022,266-378,58-91,736828-868857,622792-694076,6767592127-6767717303,2920-3656,8811329-8931031,107384-147042,941220-969217,3-17,360063-562672,7979763615-7979843972,1890-2660,23170346-23308802";

    let ranges = ranges_string
        .split(",")
        .map(|range| {
            range
                .split("-")
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut sum = 0;
    for range in ranges {
        let invalid_ids = find_invalid_ids(range[0], range[1]);
        for num in invalid_ids {
            sum += num;
        }
    }
    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11_22() {
        // Should be invalid for 11 and 22 (11 is odd, but by supplied instructions treat as invalid for test purposes)
        let expected = vec![11, 22];
        let actual = find_invalid_ids(11, 22);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_95_115() {
        // Only 99 is invalid in this range (99 is odd, should be invalid for test, though logic in main would not consider odd)
        let expected = vec![99];
        let actual = find_invalid_ids(95, 115);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_998_1012() {
        let expected = vec![1010];
        let actual = find_invalid_ids(998, 1012);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1188511880_1188511890() {
        let expected = vec![1188511885];
        let actual = find_invalid_ids(1188511880, 1188511890);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_222220_222224() {
        let expected = vec![222222];
        let actual = find_invalid_ids(222220, 222224);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1698522_1698528() {
        // Contains no invalid IDs
        let expected: Vec<i64> = vec![];
        let actual = find_invalid_ids(1698522, 1698528);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_446443_446449() {
        let expected = vec![446446];
        let actual = find_invalid_ids(446443, 446449);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_38593856_38593862() {
        let expected = vec![38593859];
        let actual = find_invalid_ids(38593856, 38593862);
        assert_eq!(actual, expected);
    }
}
