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

#### Day 2
First time ever using `reduce`. Nice

```python
from functools import reduce

def is_valid_id(x):
    x = str(x)
    if len(x) % 2 == 0:
        if x[:len(x)//2] == x[len(x)//2:]:
            return False
    return True
def find_invald(r):
    return [x for x in r if not is_valid_id(x)]

ranges = [(x.split("-")[0], x.split("-")[1]) for x in data_test.replace("\n","").split(",")]
ranges = [range(int(s), int(e)+1) for s,e in ranges]

invalid_ids = reduce(lambda x, y: x+find_invald(y), ranges, [])
```

#### Day 3
nothing. just ugly

#### Day 5
Wasted hours trying to be smart with itertools. In the end, solveit found a mistake in my `overlaps` function where I checked for complete overlap only if r2 inside r1 but not r1 inside r2.

```python
def overlaps(r1, r2):
    s1, e1 = r1
    s2, e2 = r2
    if r1 == r2:
        # print("same!")
        return size(r1)
    elif s1<=s2 and e1>=e2:
        # print("complete OL: r2 inside r1", r1, r2)
        return size(r2)
    elif s1>=s2 and e1<=e2:
        # print("complete OL: r1 inside r2", r1, r2)
        return size(r2)
    elif s1<s2 and e1<e2 and e1>=s2:
        # print("OL right", r1, r2)
        return e1-s2+1
    elif s1>s2 and e1>e2 and e2>=s1:
        # print("OL LEFT", r1, r2)
        return e2-s1+1
    return 0
```
