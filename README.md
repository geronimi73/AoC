## Advent of Code https://adventofcode.com/

### Lessons Learned

#### Day 1
Don't cram everything into one function, break it down into simpler problems.

```python
def get_next_pos(pos, lr, first_pos = 0, last_pos = 99):
    next_pos = pos - 1 if lr == "L" else pos + 1
    if next_pos < first_pos:
        next_pos = last_pos
    elif next_pos > last_pos:
        next_pos = first_pos
    return next_pos
def apply_op(pos, op):
    lr, iters = op
    positions = []
    for i in range(iters):
        pos = get_next_pos(pos, lr)
        positions.append(pos)
    return positions
```