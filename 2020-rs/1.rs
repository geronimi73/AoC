
fn get_test_input() -> &'static str {
	"1721
979
366
299
675
1456"
}

use std::fs;

fn get_input() -> String {
	fs::read_to_string("1_input.txt").unwrap()
}

fn good_combo(num1: i64, num2: i64) -> bool {
	num1 + num2 == 2020
}

fn good_combo_3args(num1: i64, num2: i64, num3: i64) -> bool {
	num1 + num2 + num3 == 2020
}

fn part2() {
	let input = get_input();
	let numbers: Vec<i64> = input.split("\n").filter_map(|x| x.trim().parse().ok()).collect();
	for n1 in 0..numbers.len() {
		for n2 in n1+1..numbers.len() {
			for n3 in n2+1..numbers.len() {
				if good_combo_3args(numbers[n1], numbers[n2], numbers[n3]) {
					println!("{}", numbers[n1] * numbers[n2] * numbers[n3]);
				}
			}
		}
	}
}

fn part1() {
	let input = get_input();
	let numbers: Vec<i64> = input.split("\n").filter_map(|x| x.trim().parse().ok()).collect();
	for n1 in 0..numbers.len() {
		for n2 in n1+1..numbers.len() {
			if good_combo(numbers[n1], numbers[n2]) {
				println!("{}", numbers[n1] * numbers[n2]);
			}
		}
	}
}

fn main() {
	part2();
}
