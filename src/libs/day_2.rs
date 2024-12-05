use super::file_handler::read_lines;

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

        println!("Levels: {:?}", levels);

        if levels.is_sorted_by(|a, b| a <= b && ((1..=3).contains(&(b - a)))) {
            println!("Safe report: {:?}", levels);
            safe_reports += 1;
        } else if levels.is_sorted_by(|a, b| a >= b && ((1..=3).contains(&(a - b)))) {
            println!("Safe report: {:?}", levels);
            safe_reports += 1;
        } else {
            println!("Not a safe report");
        }
    }

    println!("Safe reports: {safe_reports}");
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_run() {
        let input = format!("{}/src/libs/tests/day_2.txt", env!("CARGO_MANIFEST_DIR"));
        let result = run(&input);
        assert_eq!(result, 2);
    }
}
