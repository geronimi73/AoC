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
