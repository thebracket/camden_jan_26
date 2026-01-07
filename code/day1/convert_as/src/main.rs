#![warn(clippy::pedantic)] 

fn main() {
    let i: u16 = 257;
    let j: u8 = i as u8;
    println!("{i} {j}");
}