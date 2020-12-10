/*
- Store all possible sums in window, and update it.
- For 25 numbers, there are 300 [(25*25 - 25)/2] sums.
    0 1 2 3 4
  0   o o o o -> square - diagonal in half = 10
  1     o o o
  2       o o
  3         o
  4
- The 24th number uses the previous 25, but the 25th drops the 0th and adds the 25th.
- That means every sum associated with the 0th number gets modified by (25th - 0th)
- If we store those sums in a 1D array, how can we find the indices associated with a number?
    0 1 2 3 4
  0   0 1 2 3
  1     4 5 6
  2       7 8
  3         9
  4
- Want an iterator that maps as follows
  0: 0 1 2 3
  1: 0 4 5 6
  2: 1 4 7 8
  3: 2 5 7 9
  4: 3 6 8 9
- Verticals:
    - starts at: i-1.
    - adds: n-2, then n-3, then n-4 ...
    - i terms
- Horizontals:
    - starts at: i terms of (n-1) + (n-2) + ... = i*n - (1 + 2 + ... + i) i times = i*n - i(i+1)/2
    - adds: 1
    - n - 1 - i terms
- With that, we can use remainders to find the right numbers to modify. i.e., for 25th we modify 25 % 25 = 0th associated indices.
- To make lookup fast, we use a Map from number to count. If a number is in the set, the sum is possible.
- When count hits 0, we remove it from the set.
*/

use std::collections::HashMap;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let n = 25;

  let nums: Vec<i64> = read_to_string("data/day9input")?
    .split('\n')
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

  let mut sums: Vec<i64> = Vec::new();
  for i in 0..n {
    for j in i + 1..n {
      sums.push(nums[i] + nums[j]);
    }
  }

  let mut counts: HashMap<i64, i64> = HashMap::new();
  for sum in &sums {
    counts.entry(*sum).and_modify(|e| *e += 1).or_insert(1);
  }

  let indices: Vec<Vec<usize>> = (0..n)
    .map(|i| {
      let mut index = Vec::new();
      let mut vert = if i > 0 { i - 1 } else { 0 }; // to avoid underflow when i is 0 lol
      for c in 0..i {
        index.push(vert);
        vert += n - 2 - c;
      }
      let mut horiz = i * n - i * (i + 1) / 2;
      for _ in 0..n - 1 - i {
        index.push(horiz);
        horiz += 1;
      }
      index
    })
    .collect();

  let mut count = n;
  for x in &nums[n..] {
    if !counts.contains_key(x) {
      println!("{}", x);
      // 1124361034
      break;
    }

    for index in &indices[count % n] {
      let old_sum: i64 = sums[*index];
      let new_sum: i64 = old_sum - nums[count - n] + x;
      sums[*index] = new_sum;

      if *counts.get(&old_sum).unwrap() == 1 {
        counts.remove(&old_sum);
      } else {
        counts.entry(old_sum).and_modify(|e| *e -= 1);
      }
      counts.entry(new_sum).and_modify(|e| *e += 1).or_insert(1);
    }

    count += 1;
  }
  Ok(())
}
