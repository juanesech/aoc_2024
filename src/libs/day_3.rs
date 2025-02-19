use super::file_handler::read_lines;
use regex::Regex;

pub enum Part {
    P1,
    P2,
}

fn part_1(mem_dump: String) -> i32 {
    let mut result: i32 = 0;
    let re_mul = Regex::new(r"mul\(\d{0,3}\d,\d{0,3}\)").unwrap();
    let re_digits = Regex::new(r"\b\d{1,3}\b").unwrap();
    let matches: Vec<&str> = re_mul.find_iter(&mem_dump).map(|m| m.as_str()).collect();

    for mul in matches {
        let digit_matches: Vec<i32> = re_digits
            .find_iter(&mul)
            .map(|d| {
                println!("Digits string -> {:?}", d.as_str());
                d.as_str().parse::<i32>().unwrap()
            })
            .collect();
        result += digit_matches[0] * digit_matches[1];
    }
    result
}

fn part_2(mem_dump: String) -> i32 {
    let re_do = Regex::new(r"do\(\)").unwrap();
    let results_arr: Vec<_> = re_do
        .split(&mem_dump)
        .into_iter()
        .map(|do_it| {
            let splt: Vec<&str> = do_it.split("don't()").collect();
            part_1(splt[0].to_string())
        })
        .collect();
    println!("Results array -> {:?}", results_arr);

    results_arr.iter().sum()
}

pub fn run(input: &str, part: Part) -> i32 {
    let mut result: i32 = 0;
    for line in read_lines(input).unwrap() {
        let line = line.unwrap();
        result += match part {
            Part::P1 => part_1(line),
            Part::P2 => part_2(line),
        }
    }
    println!("{:?}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_run() {
        let input = format!("{}/src/libs/tests/day_3.txt", env!("CARGO_MANIFEST_DIR"));
        let result = run(&input, Part::P1);
        assert_eq!(result, 161);
    }

    #[test]
    fn day3_part2_run() {
        let input = format!("{}/src/libs/tests/day_3_p2.txt", env!("CARGO_MANIFEST_DIR"));
        let result = run(&input, Part::P2);
        assert_eq!(result, 48);
    }
}
