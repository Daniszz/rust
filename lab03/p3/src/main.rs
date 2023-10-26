#[derive(Debug)]
enum Error {
    OverflowAdunare,
    OverflowInmultire
}
fn add(x: u32, y: u32) -> Result<u32,Error> {
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64 {
          return Err(Error::OverflowAdunare);
    } 
          return Ok(sum as u32);
    
}
fn multiplication(x: u32, y: u32) -> Result<u32,Error> {
    let mul = x as u64 * y as u64;
    if mul > std::u32::MAX as u64 {
         return Err(Error::OverflowInmultire);
    } 
       return  Ok(mul as u32);
    
}
fn verificare() -> Result<bool,Error>
{
    let a :u32 = 4;
    let b: u32 = 5;
    let c: u32 = std::u32::MAX;
    let s1=add(a,b)?;
    let s2=add(a,c)?;
    let s3=multiplication(a,b)?;
    let s4=multiplication(a,c)?;
    return Ok(s1!=s2 && s3!=s4);

}
fn main() {
    match verificare() 
    {
        Ok(x) =>println!("value: {}",x),
        Err(e) => eprintln!("{:?}",e)
    }
}
