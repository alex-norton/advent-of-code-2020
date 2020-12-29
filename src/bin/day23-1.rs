use std::fs::read_to_string;

/**
 * Man this is messy
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = read_to_string("data/day23input")?
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let max = buffer.len();
    let mut cur_pos = 0;
    for _ in 0..100 {
        let cur_val = buffer[cur_pos];
        let mut remove_pos = (cur_pos + 1) % buffer.len();
        let one = buffer.remove(remove_pos);
        if remove_pos < cur_pos {
            cur_pos -= 1;
        } else {
            remove_pos = (cur_pos + 1) % buffer.len();
        }
        let two = buffer.remove(remove_pos);
        if remove_pos < cur_pos {
            cur_pos -= 1;
        } else {
            remove_pos = (cur_pos + 1) % buffer.len();
        }
        let three = buffer.remove(remove_pos);
        if remove_pos < cur_pos {
            cur_pos -= 1;
        }
        let mut target = shift_target(cur_val, max);
        while target == one || target == two || target == three {
            target = shift_target(target, max);
        }
        let target_pos = (buffer.iter().position(|&x| x == target).unwrap() + 1) % buffer.len();
        buffer.insert(target_pos, three);
        buffer.insert(target_pos, two);
        buffer.insert(target_pos, one);
        if target_pos <= cur_pos {
            cur_pos += 3;
        }
        cur_pos = (cur_pos + 1) % buffer.len();
    }
    let one_pos = buffer.iter().position(|&x| x == 1).unwrap();
    for i in 0..max - 1 {
        print!("{}", buffer[(one_pos + 1 + i) % max]);
    }
    print!("\n");

    Ok(())
}

fn shift_target(i: usize, m: usize) -> usize {
    let ret = (i as isize) - 1;
    if ret == 0 {
        m
    } else {
        ret as usize
    }
}
