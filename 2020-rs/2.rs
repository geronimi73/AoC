use std::fs;

fn _get_input() -> String {
	String::from("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc")
}

fn get_input() -> String {
	fs::read_to_string("2_input.txt").unwrap()
}


fn parse_policy(line: &str) -> (i16, i16, char) {
	let policy: &str = line.split(":").next().unwrap();
	let mut policy_iter = policy.split(" ");
	let mut minmax = policy_iter.next().unwrap().split("-");
	let min: i16	= minmax.next().unwrap().trim().parse().unwrap();
	let max: i16	= minmax.next().unwrap().trim().parse().unwrap();
	let c = policy_iter.next().unwrap().chars().next().unwrap();
	(min, max, c)
}

fn parse_pw(line: &str) -> &str {
	line.split(" ").last().unwrap()
}

fn pw_valid(min: i16, max: i16, c: char, pw: &str) -> bool {
	let cnt = pw.matches(c).count() as i16;
	cnt >= min && cnt <= max
}

// is password valid?
// c is a single character
fn pw_valid_pt2(idx1: i16, idx2: i16, c: char, pw: &str) -> bool {
	(pw.chars().nth(idx1 as usize - 1) == Some(c)) ^ (pw.chars().nth(idx2 as usize - 1) == Some(c))
}

fn part1(input: &str) {
	let mut cnt_correct = 0;

	for line in input.lines() {
		let (min, max, c) = parse_policy(line);
		let pw = parse_pw(line);
		let pw_valid = pw_valid(min, max, c, pw);
		if pw_valid {
			cnt_correct += 1
		}
		// println!("{} {}", pw, pw_valid)
	}	
	println!("{}", cnt_correct)
}

fn part2(input: &str) {
	let mut cnt_correct = 0;

	for line in input.lines() {
		let (min, max, c) = parse_policy(line);
		let pw = parse_pw(line);
		let pw_valid = pw_valid_pt2(min, max, c, pw);
		if pw_valid {
			cnt_correct += 1;
			// println!("{}", pw);
		}
		// println!("{} {}", pw, pw_valid)
	}	
	println!("{}", cnt_correct)
}

fn main() {
	let input_str = get_input();
	// part1(&input_str);
	part2(&input_str);
	// let tst = String::from("abcdefgh");
	// println!("{}", &tst[0]);
}