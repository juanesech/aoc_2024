use super::file_handler::read_lines;

enum Order {
    Ascendent,
    Descendent,
}

fn dampener(lvls: Vec<i32>, order: Order) -> Vec<i32> {
    match order {
        Order::Ascendent => {
            let mut dropped = false;
            lvls.windows(2)
                .filter(|w| {
                    if w[0] <= w[1] && !dropped {
                        dropped = true;
                        return false;
                    }
                    true
                })
                .map(|w| w[0])
                .collect()
        }
        Order::Descendent => {
            let mut dropped = false;
            lvls.windows(2)
                .filter(|w| {
                    if w[0] >= w[1] && !dropped {
                        dropped = true;
                        return false;
                    }
                    true
                })
                .map(|w| w[0])
                .collect()
        }
    }
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

        println!("Levels: {:?}", levels);
        // println!("Dampener level: {:?}", dampener(&levels));

        if asc_sorted(&levels) {
            println!("Safe report: {:?}", levels);
            safe_reports += 1;
        } else if desc_sorted(&levels) {
            println!("Safe report: {:?}", levels);
            safe_reports += 1;
        } else if levels.is_sorted() {
            let dlvls = dampener(levels, Order::Ascendent);
            if asc_sorted(&dlvls) {
                println!("Dampenered levels: {:?}", dlvls);
                safe_reports += 1;
            }
        } else if levels.is_sorted_by(|a, b| b < a) {
            let dlvls = dampener(levels, Order::Descendent);
            if desc_sorted(&dlvls) {
                println!("Dampenered levels: {:?}", dlvls);
                safe_reports += 1;
            }
        }
    }

    println!("Safe reports: {safe_reports}");
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_run() {
        let input = format!("{}/src/libs/tests/day_2.txt", env!("CARGO_MANIFEST_DIR"));
        let result = run(&input);
        assert_eq!(result, 2);
    }
    #[test]
    fn day2_part2_run() {
        let input = format!("{}/src/libs/tests/day_2.txt", env!("CARGO_MANIFEST_DIR"));
        let result = run(&input);
        assert_eq!(result, 4);
    }
}
