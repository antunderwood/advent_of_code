fn find_invalid_ids(start: i64, end: i64) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    for num in start..=end {
        if num >= 10 {
            let num_len = num.to_string().len();
            for i in 1..=num_len / 2 {
                // can number be split into equal parts of length i?
                if num_len % i == 0 {
                    let mut num_parts = vec![];
                    let mut j = 0;
                    while j + i <= num.to_string().len() {
                        num_parts.push(num.to_string()[j..j+i].parse::<i64>().unwrap());
                        j += i;
                    }
                    if !num_parts.is_empty() && num_parts.iter().all(|&x| x == num_parts[0]) {
                        invalid_ids.push(num);
                        break;
                    }
                }
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
        // Should be invalid for 11 and 22
        let expected = vec![11, 22];
        let actual = find_invalid_ids(11, 22);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_95_115() {
        // Should be invalid for 99 and 111
        let expected = vec![99, 111];
        let actual = find_invalid_ids(95, 115);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_998_1012() {
        // Should be invalid for 999 and 1010
        let expected = vec![999, 1010];
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

    #[test]
    fn test_565653_565659() {
        let expected = vec![565656];
        let actual = find_invalid_ids(565653, 565659);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_824824821_824824827() {
        let expected = vec![824824824];
        let actual = find_invalid_ids(824824821, 824824827);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2121212118_2121212124() {
        let expected = vec![2121212121];
        let actual = find_invalid_ids(2121212118, 2121212124);
        assert_eq!(actual, expected);
    }

 
}
