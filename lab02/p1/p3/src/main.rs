fn add_space(s: &mut String, y: u32) {
    let mut a = y;
    while a > 0 {
        s.push(' ');
        a -= 1;
    }
}
fn add_str(s: &mut String, x: &str) {
    s.push_str(x);
}
fn add_integer(s: &mut String, x: u32) {
    let mut numar = String::new();
    let mut a: u32 = x;
    let mut invers = 0;
    while a > 0 {
        invers = invers * 10 + a % 10;
        a = a / 10;
    }

    let mut counter = 0;
    while invers > 0 {
        if counter > 0 && counter % 3 == 0 {
            numar.push('_');
        }
        let c = ((invers % 10) as u8 + b'0') as char;
        numar.push(c);
        invers = invers / 10;
        counter += 1;
    }

    s.push_str(&numar);
}
fn add_float(s: &mut String, x: f64) {
    let numar = x.to_string();
    s.push_str(&numar);
}
fn main() {
    let mut result = String::new();

    add_space(&mut result, 40);
    add_str(&mut result, "I");
    add_space(&mut result, 1);
    add_str(&mut result, "💚\n");
    add_space(&mut result, 40);
    add_str(&mut result, "RUST.\n\n");

    add_space(&mut result, 4);
    add_str(&mut result, "Most");
    add_space(&mut result, 12);
    add_str(&mut result, "crate");
    add_space(&mut result, 6);
    add_integer(&mut result, 306437968);
    add_space(&mut result, 11);
    add_str(&mut result, "and");
    add_space(&mut result, 5);
    add_str(&mut result, "lastest");
    add_space(&mut result, 9);
    add_str(&mut result, "is\n");

    add_space(&mut result, 9);
    add_str(&mut result, "downloaded");
    add_space(&mut result, 8);
    add_str(&mut result, "has");
    add_space(&mut result, 13);
    add_str(&mut result, "downloads");
    add_space(&mut result, 5);
    add_str(&mut result, "the");
    add_space(&mut result, 9);
    add_str(&mut result, "version");
    add_space(&mut result, 4);
    add_float(&mut result, 2.038);
    add_str(&mut result, ".\n");

    println!("{}", result);
}
