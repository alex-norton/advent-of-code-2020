use std::fs::read_to_string;

// This looks like Diffie Hellman. Some googling found this prime could be vulnerable to Pohlig-Hellman.
// 20201227 = 2 * 3 * 29 * 116099
// In hindsight, that googling was completely unnecessary. :)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nums = read_to_string("data/day25input")?
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (card_pub, door_pub) = (nums[0], nums[1]);
    let card_loop = dl(card_pub, 7, 20201227);
    let door_loop = dl(door_pub, 7, 20201227);
    let enc_key1 = pow(card_pub, door_loop, 20201227);
    let enc_key2 = pow(door_pub, card_loop, 20201227);
    println!("{}", enc_key1);
    println!("{}", enc_key2);
    Ok(())
}

// g^x = y mod m, returns x
// Could use baby steps giant steps, but too lazy
fn dl(y: usize, g: usize, m: usize) -> usize {
    let mut cur = g % m;
    let mut it = 1;
    while cur != y {
        cur *= g;
        cur %= m;
        it += 1;
    }
    it
}

// g^x = y mod m, returns y
// Could use fast exponentiation, but too lazy
fn pow(g: usize, x: usize, m: usize) -> usize {
    let mut cur = g;
    for _ in 0..x - 1 {
        cur *= g;
        cur %= m;
    }
    cur
}
