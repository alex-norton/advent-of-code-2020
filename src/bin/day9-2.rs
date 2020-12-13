use std::cmp;
use std::fs::read_to_string;
/* Target: 1124361034
   Easy solution: Consider all contiguous sums "ending" at an index
   - starting at the index and work backwards up the list, summing as you go.
   - all inputs are positive, so if you exceed the target you can stop early
   - O(n^2) "re-sums" many chunks repeatedly.
   - no need to do backwards, just start and go forwards
   - can probably do better with a sliding window?
   - TODO: convince yourself sliding window doesn't miss a sum, by contradiction
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = 1124361034;

    let nums: Vec<i64> = read_to_string("data/day9input")?
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    for x in 0..nums.len() {
        let mut total = nums[x];
        let mut min = nums[x];
        let mut max = nums[x];
        for y in x + 1..nums.len() {
            if total == target {
                println!("{}, {}, {}", min, max, min + max);
                return Ok(());
            }
            max = cmp::max(max, nums[y]);
            min = cmp::min(min, nums[y]);
            total += nums[y];
            if total > target {
                break;
            }
        }
    }

    Ok(())
}
