use std::fs::read_to_string;
// https://doc.rust-lang.org/std/collections/struct.VecDeque.html
fn main() -> Result<(), Box<dyn std::error::Error>> {
    read_to_string("data/test");
    Ok(())
}
