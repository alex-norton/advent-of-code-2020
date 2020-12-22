use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sum = read_to_string("data/day18input")?
        .lines()
        .map(|x| calculate(&mut x.chars(), false))
        .sum::<u64>();
    println!("{}", sum);
    Ok(())
}

/*
    This is a messy hack to extend the method of Part 1.
    The intuition is that pluses should evalulate greedily,
    whereas multiplications should evaluate lazily.
    We handle the latter through recursion, but parentheses are
    already handled through recursion. In part 1, every step consumed
    a character, but that made multiplication recursion (which should return
    without consuming the right paren) and parenthesis recursion (which
    should consume the right paren) indistinguishable. To hack around that,
    there's this "consume_paren" boolean which uses chars::as_str to do the
    equivalent of peeking, to allow multiplication recursions to peek the ')'
    and return without consuming it.
*/
fn calculate(cs: &mut std::str::Chars, consume_paren: bool) -> u64 {
    let mut total: u64 = 0;
    let mut is_plus = false;
    loop {
        if !consume_paren {
            let x = cs.as_str();
            if let Some(c) = x.chars().next() {
                if c == ')' {
                    return total;
                }
            }
        }
        if let Some(c) = cs.next() {
            match c {
                ')' => return total,
                ' ' => continue,
                '+' => is_plus = true,
                '*' => total *= calculate(cs, false),
                c => {
                    let num = match c {
                        '(' => calculate(cs, true),
                        c => c.to_digit(10).unwrap() as u64,
                    };
                    if is_plus {
                        total += num;
                    } else {
                        total = num;
                    }
                }
            }
        } else {
            break;
        }
    }
    total
}
