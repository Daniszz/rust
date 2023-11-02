use std::fs;

fn verificare(cuvant: &str) -> Option<&str> {
    match cuvant {
        "pt" => Some("pentru"),
        "ptr" => Some("pentru"),
        "dl" => Some("domnul"),
        "dna" => Some("doamna"),
        _ => None,
    }
}

fn main() {
    let sir = fs::read_to_string("src/1.txt");
    match sir {
        Ok(sir) => {
            let mut nou_sir = String::new();
            for cuvant in sir.split(" ") {
                match verificare(cuvant) {
                    Some(cuvant) => {
                        nou_sir.push_str(cuvant);
                        nou_sir.push(' ');
                    }
                    None => {
                        nou_sir.push_str(cuvant);
                        nou_sir.push(' ');
                    }
                }
            }
            println!("Processed sir: {}", nou_sir);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
