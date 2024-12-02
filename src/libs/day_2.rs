use super::file_handler::read_lines;

pub fn run(input: &str) -> i32 {
    let mut safe_reports = 0;

    for line in read_lines(input).unwrap() {
        let line = line.unwrap();
        let report_nums: Vec<i32> = line
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|n| n.parse::<i32>().unwrap_or(0))
            .collect();

        if report_nums.is_sorted_by(|a, b| a <= b && b - a > 1 && b - a <= 3) {
            println!("Resport: {:?}", report_nums);
            safe_reports += 1;
        } else if report_nums.is_sorted_by(|a, b| a >= b && a - b > 1 && a - b <= 3) {
            println!("Resport: {:?}", report_nums);
            safe_reports += 1;
        }
    }

    println!("{safe_reports}");
    safe_reports
}
