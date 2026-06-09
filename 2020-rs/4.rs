// use std::any::type_name_of_val;
use std::collections::HashMap;
use std::fs;

fn get_input() -> String {
	fs::read_to_string("4_input.txt").unwrap()
}

fn _get_input() -> String {
	String::from("
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
")
}

fn parse_pp(input: &str) -> Vec<HashMap<&str, &str>> {
	let mut result = Vec::new();

	for pp_raw in input.split("\n\n").map(|x| x.trim()) {
		// I wonder: pp_raw is a &str. Who owns this var?
		// And: why does `parse_pp(pp_raw)` work here, dont i need `parse_pp(&pp_raw)` ?
		// And: what's the lifetime of the &str in the Hashmap?
		let pp: HashMap<&str, &str> = pp_raw
			.split_whitespace()
			.map(|x| x.split_once(":").unwrap())
			.collect();
		// the command above is so elegant! I love how Rust shoves the split results into the right datatype!
		result.push(pp);
	}
	result
}

fn part1(passports: &Vec<HashMap<&str, &str>>) {
	let mut valid_cnt: i32 = 0;
	let required_keys = [
		"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", 
	];

	// this feels all too pythonic to me
	for pp in passports {
		let valid = required_keys.iter().all(|key_req| pp.contains_key(key_req));
		// I wonder if there's something more idiomatic in rust than this one:
		if valid {
			valid_cnt += 1;
		}
	}
	println!("{valid_cnt}")
}

fn main() {
	let input = get_input();
	let passports = parse_pp(&input);

	part1(&passports);
}