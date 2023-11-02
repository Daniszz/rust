use std::fs;
#[derive(Debug)]
enum Error {
    NotAscii,
}
fn transformare(sir: String) -> Result<String, Error> {
    let mut nou_sir = String::new();
    for i in sir.chars() {
        let new_char;
        if i.is_ascii() {
            if (i >= 'a' && i <= 'z') || (i >= 'A' && i <= 'Z') {
                if (i >= 'a' && i <= 'm') || (i >= 'A' && i <= 'M') {
                    new_char = (i as u8 + 13) as char;
                } else {
                    new_char = (i as u8 - 13) as char;
                }
                nou_sir.push(new_char);
            } else {
                nou_sir.push(i);
            }
        } else {
            return Err(Error::NotAscii);
        }
    }
    return Ok(nou_sir);
}

fn main() {
    let sir = fs::read_to_string("src/1.txt");
    match sir {
        Ok(sir) => match transformare(sir) {
            Ok(sir) => println!("Sirul e este {}", sir),
            Err(err) => eprintln!("Eroare {:?}", err),
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
