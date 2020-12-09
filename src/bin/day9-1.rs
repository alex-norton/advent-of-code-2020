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
    - adds: n-2
    - i terms
- Horizontals:
    - starts at: i terms of (n-1) + (n-2) + ... = i*n - (1 + 2 + ... + i) i times = i*n - i(i+1)/2
    - adds: 1
    - n - 1 - i terms
- With that, we can use remainders to find the right numbers to modify. i.e., for 25th we modify 25 % 25 = 0th associated indices.
- To make lookup fast, we use a Map from number to count. If a number is in the set, the sum is possible.
- When count hits 0, we remove it from the set.
*/

fn main() {}
