I'm learning Rust
Advent of Code COVID year


## Day 1 
* `filter_map` looks like I will use it a lot, nore sure how it works exactly, especially the `ok`
* method overloading apparently does not work, every fn has to have a unique name
* I shouldnt use `unwrap` according to the Rust book but it's too convenient 

Claude's review:
* use String`.lines()` instead of `.split("\n")` 
* `.iter()` of a `Vec<i64>` yields references not values!

## Day 2 
* yeah i'm overdoing the unwrap `let min: i16 = minmax.next().unwrap().trim().parse().unwrap();`
* logical xor is `^`
* &str.`nth()` takes a `usize` which is some basic type required by a lot of functions, conversion works with `x as usize`. Seems like its type is OS-dependent, either 32 or 64 bit (?)

## Day 3
* chars().nth() is O(n)

chars() walks from the start each time, so is_tree is O(width) per call. Since your input isASCII, use as_bytes() instead — direct index, O(1):

```
  fn is_tree(pos: &Pos, grid: &[&str]) -> bool {
      let row = grid[pos.y].as_bytes();
      row[pos.x % row.len()] == b'#'
  }
```

instead of 

```
fn is_tree(pos: &Pos, grid: &Vec<&str>) -> bool {
	let chars_line_cnt = grid[pos.y as usize].chars().count();
	let mut chars_line: Chars = grid[pos.y as usize].chars();
	let c: char = chars_line.nth(pos.x as usize % chars_line_cnt).expect("invalid idx");
	// println!("{c}");
	c == '#'
}
```
* Grid coordinates can't be negative here, so usize is the natural type. Using i16 forces you
  to cast everywhere
* &String and &Vec<T> as parameters are un-idiomatic
  This is the biggest one. In Rust, you should prefer the "unsized" versions:
  - &String → &str
  - &Vec<&str> → &[&str]
