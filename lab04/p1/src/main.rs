use std::fs;

fn main() {
    let sir = fs::read_to_string("src/1.txt");
    let mut sir_size = String::new();
    let mut sir_capacity = String::new();
    let mut max_size: usize = 0;
    let mut max_capacity: usize = 0;
    match sir {
        Ok(sir) => {
            for line in sir.lines() {
                let line = line.to_string();
                let len = line.len();
                if len > max_size {
                    max_size = len;
                    sir_size = String::from(&line);
                }
                let capacity = line.capacity();
                if capacity > max_capacity {
                    max_capacity = capacity;
                    sir_capacity = String::from(&line);
                }
            }
            println!("Sirul cel mai lung in functie de bytes : {}", sir_capacity);
            println!("Sirul cel mai lung in functie de caractere: {}", sir_size);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
