// FIX the errors and FILL in the blank
// DON'T remove any code
fn main() {
    let decimal = 98.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = integer as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8);

    println!("Success!");
}