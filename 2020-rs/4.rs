// use std::any::type_name_of_val;
use std::collections::HashMap;
use std::fs;

fn get_input() -> String {
	fs::read_to_string("4_input.txt").unwrap()
}

fn _get_input() -> String {
	String::from("
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
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

fn part1(passports: &[HashMap<&str, &str>]) {
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

fn validate_pp(passport: &HashMap<&str, &str>) -> bool {
	let mut is_valid = true;

	for (&key, &val) in passport.iter() {
		match key {
			"byr" => {
				is_valid = val.chars().count() == 4 && match val.parse::<u32>() {
				    Ok(year) => (1920..=2002).contains(&year),
				    Err(_) => false,
				};
				if !is_valid {
					println!("byr invalid {}", val)
				}
			}
			"iyr" => {
				is_valid = val.chars().count() == 4 && match val.parse::<u32>() {
				    Ok(year) => (2010..=2020).contains(&year),
				    Err(_) => false,
				};
				if !is_valid {
					println!("iyr invalid {}", val)
				}
			}
			"eyr" => {
				is_valid = val.chars().count() == 4 && match val.parse::<u32>() {
				    Ok(year) => (2020..=2030).contains(&year),
				    Err(_) => false,
				};
				if !is_valid {
					println!("iyr invalid {}", val)
				}
			}
			"ecl" => {
				let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
				is_valid = valid_ecl.contains(&val);
				if !is_valid {
					println!("ecl invalid {}", val)
				}
			}
			"pid" => {
				is_valid = val.chars().count() == 9 && match val.parse::<u32>() {
				    Ok(year) => true,
				    Err(_) => false,
				};
				if !is_valid {
					println!("pid invalid {}", val)
				}
			}
			"hcl" => {
				is_valid = val.chars().next() == Some('#')  && val.chars().skip(1).all(|x| x.is_ascii_alphanumeric());
				if !is_valid {
					println!("hcl invalid {}", val)
				}
			}
			"hgt" => {
				if val.ends_with("cm") {
					is_valid = match val.trim_end_matches("cm").parse::<u32>() {
						// I dont understand this: hgt is a u32. That's on the stack. Why do I need a ref to it??
						// I thought that's not even possible
						Ok(hgt) => (150..=193).contains(&hgt),
						Err(_) => false
					};
				} else if val.ends_with("in") {
					is_valid = match val.trim_end_matches("in").parse::<u32>() {
						Ok(hgt) => (59..=76).contains(&hgt),
						Err(_) => false
					};
				} else {
					is_valid = false;
				}
				if !is_valid {
					println!("hgt invalid {}", val)
				}
			}
			// missing: hgt (Height) - a number followed by either cm or in:
				// If cm, the number must be at least 150 and at most 193.
				// If in, the number must be at least 59 and at most 76.
			_ => {
				println!("unhandled prop {}", key);
			}
		}
		if !is_valid {
			return is_valid
		}
	}
	true
}

fn part2(passports: &Vec<HashMap<&str, &str>>) {
	let mut valid_cnt: i32 = 0;
	let required_keys = [
		"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", 
	];

	// this feels all too pythonic to me
	for pp in passports {
		let all_keys = required_keys.iter().all(|key_req| pp.contains_key(key_req));
		let is_valid = validate_pp(&pp);

		if all_keys && is_valid  {
			valid_cnt += 1;
		}
	}
	println!("{valid_cnt}")
}

fn main() {
	let input = get_input();
	let passports = parse_pp(&input);

	part1(&passports);
	// part2(&passports);
}