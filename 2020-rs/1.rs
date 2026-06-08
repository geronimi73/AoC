use std::fs;

fn get_test_input() -> &'static str {
	"1721
979
366
299
675
1456"
}

fn get_input() -> String {
	fs::read_to_string("1_input.txt").unwrap()
}

fn part2(numbers: &Vec<i64>) {
	let good_combo = |x, y, z| x + y + z == 2020;

	for (idx1, &n1) in numbers.iter().enumerate() {
		for (idx2, &n2) in numbers[idx1+1..].iter().enumerate() {
			for &n3 in numbers[idx2+idx1+2..].iter() {
				if good_combo(n1, n2, n3) {
					println!("{}", n1 * n2 * n3);
				}
			}
		}
	}
}

fn part1(numbers: &Vec<i64>) {
	let good_combo = |x, y| x + y == 2020;

	for (idx1, &n1) in numbers.iter().enumerate() {
		for n2 in &numbers[idx1+1..] {
			if good_combo(n1, n2) {
				println!("{}", n1 * n2);
			}
		}
	}
}

fn main() {
	let input = get_input();
	let numbers: Vec<i64> = input.lines().filter_map(|x| x.trim().parse().ok()).collect();

	part1(&numbers);
	part2(&numbers);
}


