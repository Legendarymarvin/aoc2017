fn main() {
    let input = include_str!("./inputs/01");
    let (part1, part2) = solve(input);
    println!("{}", part1);
    println!("{}", part2);
}

fn solve(p0: &str) -> (i32, i32) {
    let numbers: Vec<i32> = p0.chars()
        .filter_map(|c| c.to_digit(10).map(|n| n as i32))
        .collect();
    let part1 = numbers.iter()
        // pair every number with the next, cycle over
        .zip(numbers.iter().cycle().skip(1))
        // only get the same ones
        .filter(|(x,y)| x == y)
        // only the first
        .map(|(x, _y)| x)
        .sum();
    let part2 = numbers.iter()
        .zip(numbers.iter().cycle().skip(numbers.len()/2))
        .filter(|(x,y)| x == y)
        .map(|(x, _y)| x)
        .sum();
    (part1, part2)
}

#[cfg(test)]

mod tests {
    use crate::solve;

    #[test]
    fn part1_might_work() {
        assert_eq!(3, solve("1122").0);
        assert_eq!(4, solve("1111").0);
        assert_eq!(0, solve("1234").0);
        assert_eq!(9, solve("91212129").0);
    }

    #[test]
    fn part2_might_work() {
        assert_eq!(6, solve("1212").1);
        assert_eq!(0, solve("1221").1);
        assert_eq!(4, solve("123425").1);
        assert_eq!(4, solve("12131415").1);
    }

    #[test]
    fn still_works() {
        assert_eq!((1203, 1146), solve(include_str!("./inputs/01")))
    }
}
