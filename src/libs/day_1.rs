use itertools::sorted;
use std::{cmp, usize};

use super::file_handler::read_lines;

fn find_all<T: PartialEq>(arr: &[T], item: &T) -> Vec<usize> {
    arr.iter()
        .enumerate()
        .filter_map(|(index, element)| if element == item { Some(index) } else { None })
        .collect()
}

pub fn run(input: &str) -> i32 {
    let mut total_distance = 0;
    let mut similarity_score: i32 = 0;

    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in read_lines(input).unwrap() {
        let line = line.unwrap();
        let splited: Vec<&str> = line.split(" ").collect();
        column1.push(splited[0].parse::<i32>().expect("Not a number"));
        column2.push(splited[1].parse::<i32>().expect("Not a number"));
    }

    let sorted_c1 = sorted(column1.into_iter());
    print!("Sorted c1: {:?}", sorted_c1);
    let sorted_c2 = sorted(column2.into_iter()).collect::<Vec<i32>>();
    print!("Sorted c2: {:?}", sorted_c2);

    sorted_c1.enumerate().for_each(|(i, v)| {
        let min = cmp::min(v, sorted_c2[i]);
        let max = cmp::max(v, sorted_c2[i]);
        total_distance += max - min;
        similarity_score += v * find_all(&sorted_c2, &v)
            .iter()
            .count()
            .try_into()
            .unwrap_or(0);
    });

    println!("Total distance: {:?}", total_distance);
    println!("Similarity score: {:?}", similarity_score);
    total_distance
}
