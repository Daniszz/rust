fn add_chars_n(x: String, y: char, z: u32) -> String {
    let mut s: String = x;
    let mut a: u32 = z;
    while a > 0 {
        s.push(y);
        a -= 1;
    }
    return s;
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }

    print!("{}", s);
}
