use std::fs;

fn main() {
    let sir: Result<String, std::io::Error> = fs::read_to_string("src/1.txt");
    match sir {
        Ok(sir) => {
            for linie in sir.lines() {
                if !linie.starts_with('#') {
                    let mut cuvant = linie.split_whitespace();
                    if let Some(sir1) = cuvant.next() {
                        if let Some(sir2) = cuvant.next() {
                            println!("{} => {}", sir2, sir1);
                        }
                    }
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
