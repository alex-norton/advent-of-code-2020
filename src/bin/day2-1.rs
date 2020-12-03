use advent::read_lines;

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    for line in read_lines("data/day1input") {
        if let Ok(ip) = line {
            let num = ip.parse().unwrap();
            nums.push(num);
        }
    }
    for num1 in &nums {
        for num2 in &nums {
            for num3 in &nums {
                if num1 + num2 + num3 == 2020 {
                    println!("{}, {}, {}", num1, num2, num3)
                    // found 780, 542, 698
                }
            }
        }
    }
}
