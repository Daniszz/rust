fn main() {
    let mut x = 0;

    while x <= 100 {
        let mut y = 0;
        while y <= 100 {
            if coprim(x, y) == 1 {
                println!("Coprime:{},{}", x, y);
            }
            y = y + 1;
        }
        x = x + 1;
    }
}
fn coprim(x: i32, y: i32) -> i32 {
    let mut a = x;
    let mut b = y;
    while a != b {
        if a > b {
            a = a - b;
        }
        if a < b {
            b = b - a;
        }
    }
    if a == 1 {
        return 1;
    }
    return 0;
}
