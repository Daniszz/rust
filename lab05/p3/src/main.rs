use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn main() {
    let date = fs::read_to_string("src/1.json");

    let mut s_tanar: Option<Student> = None;
    let mut s_batran: Option<Student> = None;

    match date {
        Ok(date) => {
            for line in date.lines() {
                if let Ok(student) = serde_json::from_str::<Student>(&line) {
                    if let Some(tanar) = &s_tanar {
                        if student.age < tanar.age {
                            s_tanar = Some(student.clone());
                        }
                    } else {
                        s_tanar = Some(student.clone());
                    }

                    if let Some(batran) = &s_batran {
                        if student.age > batran.age {
                            s_batran = Some(student.clone());
                        }
                    } else {
                        s_batran = Some(student.clone());
                    }
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    match s_tanar {
        Some(tanar) => {
            println!(
                "The youngest student is {} (Age: {}, Phone: {})",
                tanar.name, tanar.age, tanar.phone
            );
        }
        None => {
            println!("No students found.");
        }
    }

    match s_batran {
        Some(batran) => {
            println!(
                "The oldest student is {} (Age: {}, Phone: {})",
                batran.name, batran.age, batran.phone
            );
        }
        None => {
            println!("No students found.");
        }
    }
}
