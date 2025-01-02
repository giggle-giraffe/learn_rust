fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    println!("{:?}", slice);
    println!("{:?}", std::mem::size_of_val(&slice));
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");

    println!("Size of slice struct: {}", std::mem::size_of_val(&slice));        // 16 bytes
    println!("Size of one char: {}", std::mem::size_of_val(&arr[0]));          // 4 bytes
    println!("Size of data slice points to: {}", slice.len() * 4);             // 8 bytes
}
