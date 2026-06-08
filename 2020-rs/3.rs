use std::str::Chars;
use std::fs;

fn _get_input() -> String {
	String::from(
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#")
}

fn get_input() -> String {
	fs::read_to_string("3_input.txt").unwrap()
}

struct Pos {
	x: i16,
	y: i16
}

impl Pos {
	fn move_by(&mut self, x: i16, y: i16) {
		// Pos {x: self.x + x, y: self.y+y}
		self.x += x;
		self.y += y;
	}
}

fn is_tree(pos: &Pos, grid: &Vec<&str>) -> bool {
	let chars_line_cnt = grid[pos.y as usize].chars().count();
	let mut chars_line: Chars = grid[pos.y as usize].chars();
	let c: char = chars_line.nth(pos.x as usize % chars_line_cnt).expect("invalid idx");
	// println!("{c}");
	c == '#'
}

fn part1(input: &String, slope_x: i16, slope_y: i16) -> i16 {
	let grid: Vec<&str> = input.lines().collect();
	let mut curr_pos = Pos {x: 0, y: 0};
	let mut tree_cnt = 0;
	while (curr_pos.y as usize) < grid.len() {
		if is_tree(&curr_pos, &grid) {
			tree_cnt += 1;
			// println!("{}/{} is a tree", &curr_pos.x, &curr_pos.y);
		} 
		curr_pos.move_by(slope_x, slope_y);		
	}
	println!("Trees: {:?}", tree_cnt);
	tree_cnt
}

fn part2(input: &String) {
	let mut tree_cnt: i64 = 1;
	let slopes = [
		(1, 1),
		(3, 1),
		(5, 1),
		(7, 1),
		(1, 2),
	];

	for (slope_x, slope_y) in slopes {
		tree_cnt *= part1(&input, slope_x, slope_y) as i64;
	}
	println!("Trees (part2): {:?}", tree_cnt);
}


fn main() {
	let input = get_input();
	part1(&input, 3, 1);
	part2(&input);
}




