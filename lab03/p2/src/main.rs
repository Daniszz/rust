fn add(x: u32, y: u32) -> u32 {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        panic!("ahhhhhhh");
    } else {
        return sum as u32;
    }
}
fn multiplication(x: u32, y: u32) -> u32 {
    let mul = x as u64 * y as u64;
    if mul > std::u32::MAX as u64 {
        panic!("ahhhhhhh");
    } else {
        return mul as u32;
    }
}
fn main() {
    let x = std::u32::MAX - 4;
    let y = 3;
    println!("{}", add(x, y));
    println!("{}", multiplication(x, y));
}
