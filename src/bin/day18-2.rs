use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sum = read_to_string("data/test")?
        .lines()
        .map(|x| calculate(0, &mut x.chars(), false))
        .sum::<u64>();
    println!("{}", sum);
    Ok(())
}

fn calculate(num: u64, cs: &mut std::str::Chars, consume_paren: bool) -> u64 {
    let mut total: u64 = num;
    let mut op: Option<char> = None;
    loop {
        if !consume_paren {
            let x = cs.as_str();
            if let Some(c) = x.chars().next() {
                if c == ')' {
                    println!("Peeked ), returning {}", total);
                    return total;
                }
            }
        }
        if let Some(c) = cs.next() {
            match c {
                ')' => {
                    println!("Consumed ), returning {}", total);
                    return total;
                }
                ' ' => continue,
                '+' => op = Some('+'),
                '*' => {
                    total *= {
                        println!("Multiply, recursing");
                        let val = calculate(num, cs, false);
                        println!("Multiplying {} by {}", total, val);
                        val
                    }
                }
                c => {
                    let num = match c {
                        '(' => {
                            println!("Consumed (, recursing");
                            calculate(0, cs, true)
                        }
                        c => c.to_digit(10).unwrap() as u64,
                    };
                    match op {
                        None => total = num,
                        Some('+') => {
                            println!("Adding {} to {}", num, total);
                            total += num
                        }
                        _ => panic!(),
                    }
                }
            }
        } else {
            break;
        }
    }
    println!("End of string, returning {}", total);
    total
}
