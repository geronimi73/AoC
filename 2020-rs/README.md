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

## Day 4
* Above lesson again. You can pass a Vec to a function which takes `&[ .. ]`

E.g. define `fn part1(passports: &[HashMap<&str, &str>]) {`, but pass a `Vec<HashMap>`. OK? How does that work?

* A pattern I love:

```
is_valid = val.chars().count() == 4 && match val.parse::<u32>() {
    Ok(year) => (1920..=2002).contains(&year),
    Err(_) => false,
};
if !is_valid {
  println!("byr invalid {}", val)
}
```

.. can (should!) be replaced with 

```
val.parse::<u32>().is_ok_and(|y| (1920..=2002).contains(&y))
```

* And another one of my python-translations:

```
match val.parse::<u32>() {
    Ok(year) => true,
    Err(_) => false,
};
```

can literally replaced with `.is_ok()` 

* Complete example:

Replace 

```
is_valid = val.chars().count() == 9 && match val.parse::<u32>() {
    Ok(year) => true,
    Err(_) => false,
};
```

with `val.len() == 9 && val.chars().all(|c| c.is_ascii_digit())`

* And another nice pattern Claude suggested to use:

```
          "hcl" => val.strip_prefix('#')
                      .is_some_and(|rest| rest.len() == 6
                          && rest.chars().all(|c| c.is_ascii_hexdigit())),
```
* stop translating python



## In between lessons

### Error handling in Rust (ie. where are the python exceptions)
* There's two basic types for returning results: `Option` and `Result`
* `Option<T>` — "Is there a value or not?" either `Some` or `None`
* `Result<T, E>` — "Did it succeed or fail, and why?"
* `Option` is used in cases where None is expected and OK, while `Result` allows to dig deeper into why there is nothing, ie. this should work but something might go wrong and if that happens you also get an Error (compare to `Option`: None is just None, without any details given)
* So Python's `None` would be an `Option`. Optional attr in python classes for example which are `None` by default are modeled as `Option` in Rust
* String splitting and parsing can apparently combined into **a single**  `if` statement. Very convenient

```rs
if      let Some((name, age)) = s.split_once(",") 
    &&  let Ok(age) = age.parse()
    &&  name.trim().chars().count() > 0
{
    Self {
        name: String::from(name.trim()),
        age
    }
} else {
    Self::default()
}
```

Suggested improved version by Opus:

```rs
if let Some((name, age)) = s.split_once(',')
    && let Ok(age) = age.parse()
    && let name = name.trim()
    && !name.is_empty()
{
    Self {
        name: String::from(name),
        age,
    }
} else {
    Self::default()
}
```