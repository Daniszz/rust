fn main() {
    let mut x = 0;
    while x <= 100 {
        if prim(x) == 1 {
            println!("Numarul {} este prim.", x);
        }
        x = x + 1;
    }
}
fn prim(n: i32) -> i32 {
    if n <= 1 {
        return 0;
    } else {
        let mut i: i32 = 2;
        while i <= n / 2 {
            if n % i == 0 {
                return 0;
            }
            i = i + 1;
        }
    }
    return 1;
}
