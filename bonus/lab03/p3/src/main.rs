use thiserror::Error;
#[derive(Error, Debug)]
enum Error {
    #[error("the addition of {0} and {1} overflows")]
    OverflowAdunare(u32, u32),
    #[error("the multiplication of {0} and {1} overflows")]
    OverflowInmultire(u32, u32),
}
fn add(x: u32, y: u32) -> Result<u32, Error> {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
        return Err(Error::OverflowAdunare(x, y));
    }
    return Ok(sum as u32);
}
fn multiplication(x: u32, y: u32) -> Result<u32, Error> {
    let mul = x as u64 * y as u64;
    if mul > std::u32::MAX as u64 {
        return Err(Error::OverflowInmultire(x, y));
    }
    return Ok(mul as u32);
}
fn verificare() -> Result<bool, Error> {
    let a: u32 = 4;
    let b: u32 = 5;
    let c: u32 = std::u32::MAX - 6;
    let s1 = add(a, b)?;
    let s2 = add(a, c)?;
    let s3 = multiplication(a, b)?;
    let s4 = multiplication(a, c)?;
    return Ok(s1 != s2 && s3 != s4);
}
fn main() {
    match verificare() {
        Ok(x) => println!("value: {}", x),
        Err(e) => println!("err :{}", e),
    }
}
