use super::file_handler::read_lines;

fn dampener_safe(lvls: Vec<i32>) -> bool {
    if asc_sorted(&lvls) || desc_sorted(&lvls) {
        return true;
    }
    for i in 0..lvls.len() {
        let mut lvls_vec = lvls.clone();
        lvls_vec.remove(i);
        if asc_sorted(&lvls_vec) || desc_sorted(&lvls_vec) {
            println!(" dampennered -- safe");
            return true;
        }
    }
    false
}

fn asc_sorted(lvls: &Vec<i32>) -> bool {
    if lvls.is_sorted_by(|a, b| a <= b && ((1..=3).contains(&(b - a)))) {
        return true;
    }
    false
}

fn desc_sorted(lvls: &Vec<i32>) -> bool {
    if lvls.is_sorted_by(|a, b| a >= b && ((1..=3).contains(&(a - b)))) {
        return true;
    }
    false
}

pub fn run(input: &str) -> i32 {
    let mut safe_reports = 0;

    for line in read_lines(input).unwrap() {
        let line = line.unwrap();
        let levels: Vec<i32> = line
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|n| n.parse::<i32>().unwrap_or(0))
            .collect();

        print!("Levels: {:?}", levels);
        if dampener_safe(levels) {
            safe_reports += 1;
        } else {
            println!(" -- unsafe")
        }
        println!("");
    }

    println!("Safe reports: {safe_reports}");
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn day2_part1_run() {
    //     let input = format!("{}/src/libs/tests/day_2.txt", env!("CARGO_MANIFEST_DIR"));
    //     let result = run(&input);
    //     assert_eq!(result, 2);
    // }
    #[test]
    fn day2_part2_run() {
        let input = format!("{}/src/libs/tests/day_2.txt", env!("CARGO_MANIFEST_DIR"));
        let result = run(&input);
        assert_eq!(result, 4);
    }
}
