use std::fs::read_to_string;

/**
 * Not fast enough. Probably wasting time shifting everything in the intermediate steps.
 * Thoughts:
 * - better to implement yourself, with an array and slices, no re-sizing.
 * - slices have copy_from_slice, copy_within with should be the most efficient memory moves
 * - use the right size unsigned ints
 * - then you only need to shift correct blocks of memory.
 * - consider how big a normal shift could be. if it's large, maybe easier to wrap around more often? (rotate_left, rotate_right)
 * - if moving memory in blocks is cheap, then whenever you "wrap around", might be easier to shift everything to left or right edge in two moves (make contiguous)
 * - if you ever spot a loop (identical cur_pos and buffer) you can short circuit, can catch with set that just stores hashes, not elements
 * - can you avoid finding target in each loop?
 */
const MS: usize = 9;
const MI: u32 = 9;
const I: usize = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer_v = vec![0; MS];
    read_to_string("data/test")?
        .chars()
        .enumerate()
        .for_each(|(i, c)| buffer_v[i] = c.to_digit(10).unwrap());
    for i in 10..=MS {
        buffer_v[i - 1] = i as u32;
    }
    // this is to avoid allocating all this on stack at once
    let mut buffer = buffer_v.into_boxed_slice();
    let mut cur_pos = 0;
    println!("\nbuffer: {:?}", buffer);
    for i in 0..I {
        let cur_val = buffer[cur_pos];
        println!("cur_val: {}", cur_val);
        println!("cur_pos: {}", cur_pos);
        let mut elem = [0; 3];
        elem[0] = buffer[(cur_pos + 1) % MS];
        elem[1] = buffer[(cur_pos + 2) % MS];
        elem[2] = buffer[(cur_pos + 3) % MS];
        println!("taken: {:?}", elem);
        let target_val = find_target(cur_val, &elem);
        println!("target_val: {}", target_val);
        let mut target_pos = buffer.iter().position(|&x| x == target_val).unwrap();
        println!("target_pos: {}", target_val);
        if target_pos + 3 >= MS {
            // amount to rot left to put 3 at end of buffer
            let shift_amount = target_pos + 4 - MS;
            println!("rotated at {}", i);
            buffer.rotate_left(shift_amount);
            target_pos = MS - 4;
            cur_pos = shift(cur_pos, shift_amount);
            println!("rotated buffer: {:?}", buffer);
            println!("rotated cur_pos: {}", cur_pos);
            println!("rotated target_pos: {}", target_pos);
        }
        if cur_pos < target_pos {
            buffer.copy_within((cur_pos + 4)..=target_pos, cur_pos + 1);
            buffer[(target_pos - 2)..(target_pos + 1)].copy_from_slice(&elem);
        } else if target_pos < cur_pos {
            buffer.copy_within((target_pos + 1)..=cur_pos, target_pos + 4);
            buffer[(target_pos + 1)..(target_pos + 4)].copy_from_slice(&elem);
        } else {
            println!("target = cur");
        }
        println!("\nbuffer: {:?}", buffer);

        cur_pos = (cur_pos + 1) % MS;
    }
    let one_pos = buffer.iter().position(|&x| x == 1).unwrap();
    /*
    println!(
        "{}",
        buffer[(one_pos + 1) % MS] * buffer[(one_pos + 2) % MS]
    );
    */
    for i in 0..MS - 1 {
        print!("{}", buffer[(one_pos + 1 + i) % MS]);
    }
    print!("\n");

    Ok(())
}

fn shift(start: usize, amount: usize) -> usize {
    let mut ret = start;
    for _ in 0..amount {
        if ret == 0 {
            ret = MS - 1;
        } else {
            ret -= 1;
        }
    }
    ret
}

fn find_target(start: u32, avoid: &[u32]) -> u32 {
    let mut ret = start;
    if ret == 1 {
        ret = MI;
    } else {
        ret -= 1;
    }
    while let Some(_) = avoid.iter().find(|&&x| x == ret) {
        if ret == 1 {
            ret = MI;
        } else {
            ret -= 1;
        }
    }
    ret
}
