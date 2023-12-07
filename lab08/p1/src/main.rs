use std::collections::HashMap;
use std::fs;

fn verificare(c: char) -> bool {
    c.is_whitespace() || c.is_ascii_punctuation()
}

fn main() {
    let sir = fs::read_to_string("src/1.txt");

    let mut nr_cuv = HashMap::new();

    match sir {
        Ok(sir) => {
            let mut actual = String::new();

            for c in sir.chars() {
                if verificare(c) {
                    if !actual.is_empty() {
                        let cuvant = actual.to_lowercase();
                        let count = nr_cuv.entry(cuvant).or_insert(0);
                        *count += 1;
                        actual.clear();
                    }
                } else {
                    actual.push(c);
                }
            }

            if !actual.is_empty() {
                let cuvant = actual.to_lowercase();
                let count = nr_cuv.entry(cuvant).or_insert(0);
                *count += 1;
            }

            let mut nr_cuv_vec: Vec<_> = nr_cuv.into_iter().collect();

            nr_cuv_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

            for (cuvant, count) in nr_cuv_vec {
                println!("{:<10} => {}", cuvant, count);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
