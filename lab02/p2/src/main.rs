fn add_chars_n(s: &mut String, y: char, z: u32) {
    let mut a = z;
    while a > 0 {
        s.push(y);
        a -= 1;
    }
}
fn main() {
    let mut s: String = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}
