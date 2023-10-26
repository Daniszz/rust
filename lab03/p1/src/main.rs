fn prim(x: u16) -> bool {
    let mut d: u16 = 2;
    while d <= x / 2 {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    return true;
}
fn next_prime(x: u16) -> Option<u16> {
    let mut primx: u32 = x as u32;
    while !prim(primx as u16) {
        primx += 1;
    }
    if primx > std::u16::MAX as u32 {
        return None;
    } else {
        return Some(primx as u16);
    }
}

fn main() {
    let mut x: u16 = 65000;
    loop {
        match next_prime(x) {
            Some(value) => println!("Numar prim: {value}"),
            None => {
                println!("None");
                break;
            }
        }
        x += 1;
    }
}
