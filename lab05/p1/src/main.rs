use std::fs;
#[derive(Clone)]

struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn main() {
    let date = fs::read_to_string("src/1.txt");
    let mut s1 = Student {
        name: String::new(),
        phone: String::new(),
        age: 0,
    };
    let mut s_tanar = Student {
        name: String::new(),
        phone: String::new(),
        age: 0,
    };
    let mut s_batran = Student {
        name: String::new(),
        phone: String::new(),
        age: 0,
    };
    let mut min = 100;
    let mut max = 0;

    match date {
        Ok(date) => {
            for line in date.lines() {
                let mut ok: i32 = 0;
                for cuvant in line.split(",") {
                    if ok == 0 {
                        s1.name.push_str(cuvant);
                        ok += 1;
                    } else if ok == 1 {
                        s1.phone.push_str(cuvant);
                        ok += 1;
                    } else {
                        s1.age = cuvant.parse().unwrap();
                    }
                }
                if (min > s1.age) {
                    s_tanar = s1.clone();
                    min = s1.age;
                }
                if (max < s1.age) {
                    s_batran = s1.clone();
                    max = s1.age;
                }
                s1 = Student {
                    name: String::new(),
                    phone: String::new(),
                    age: 0,
                };
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    println!("The oldest student is {}", s_batran.name);
    println!("The youngest student is{}", s_tanar.name);
}
