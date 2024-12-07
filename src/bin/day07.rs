use std::io::{stdin, BufRead, BufReader};

#[derive(Debug, Clone)]
struct Calibration {
    target: usize,
    numbers: Vec<usize>,
}

fn main() {
    let reader = BufReader::new(stdin());
    let mut calibrations = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (target, numbers) = line.split_once(":").unwrap();
        let target: usize = target.parse().unwrap();
        let numbers: Vec<usize> = numbers
            .trim()
            .split(" ")
            .map(|n| (n.trim()).parse().unwrap())
            .collect();
        calibrations.push(Calibration { target, numbers });
    }

    part1(&calibrations);
    part2(&calibrations);
}

fn is_solvable(target: usize, numbers: &[usize], allow_concat: bool) -> bool {
    if numbers.len() == 0 {
        return target == 0;
    }

    let last = numbers.last().unwrap();
    let remaining = &numbers[..numbers.len() - 1];
    if target < *last {
        return false;
    }

    let add_target = target - last;
    if is_solvable(add_target, remaining, allow_concat) {
        return true;
    }

    if target % last == 0 {
        let mul_target = target / last;
        if is_solvable(mul_target, remaining, allow_concat) {
            return true;
        }
    }

    // Concat check
    if allow_concat {
        let target_str = target.to_string();
        let end_str = last.to_string();
        if let Some(concat_target_str) = target_str.strip_suffix(&end_str) {
            let concat_target: usize = concat_target_str.parse().unwrap_or(0);
            if is_solvable(concat_target, remaining, allow_concat) {
                return true;
            }
        }
    }

    return false;
}

fn part1(calibrations: &[Calibration]) {
    let result: usize = calibrations
        .iter()
        .filter_map(|cal| is_solvable(cal.target, &cal.numbers, false).then_some(cal.target))
        .sum();

    println!("Part 1: {}", result);
}

fn part2(calibrations: &[Calibration]) {
    let result: usize = calibrations
        .iter()
        .filter_map(|cal| is_solvable(cal.target, &cal.numbers, true).then_some(cal.target))
        .sum();

    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solvable() {
        assert!(is_solvable(190, &vec![10, 19], false));
        assert!(is_solvable(3267, &vec![81, 40, 27], false));
        assert!(!is_solvable(83, &vec![17, 5], false));
        assert!(!is_solvable(156, &vec![15, 6], false));
        assert!(!is_solvable(7290, &vec![6, 8, 6, 15], false));
        assert!(!is_solvable(161011, &vec![16, 10, 13], false));
        assert!(!is_solvable(192, &vec![17, 8, 14], false));
        assert!(!is_solvable(21037, &vec![9, 7, 18, 13], false));
        assert!(is_solvable(292, &vec![11, 6, 16, 20], false));
    }

    #[test]
    fn test_is_solvable_with_concat() {
        assert!(is_solvable(190, &vec![10, 19], true));
        assert!(is_solvable(3267, &vec![81, 40, 27], true));
        assert!(!is_solvable(83, &vec![17, 5], true));
        assert!(is_solvable(156, &vec![15, 6], true));
        assert!(is_solvable(7290, &vec![6, 8, 6, 15], true));
        assert!(!is_solvable(161011, &vec![16, 10, 13], true));
        assert!(is_solvable(192, &vec![17, 8, 14], true));
        assert!(!is_solvable(21037, &vec![9, 7, 18, 13], true));
        assert!(is_solvable(292, &vec![11, 6, 16, 20], true));
    }
}
