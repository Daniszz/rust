use std::fs;
trait ImCom {
    fn get_name(&self) -> &str;
    fn exec(&mut self, argumente: &[&str]);
}
struct ParitateCommand;
impl ImCom for ParitateCommand {
    fn get_name(&self) -> &str {
        "par"
    }

    fn exec(&mut self, args: &[&str]) {
        if let Some(arg) = args.get(0) {
            if let Ok(num) = arg.parse::<i32>() {
                if num % 2 == 0 {
                    println!("Numarul {} este par.", num);
                } else {
                    println!("Numarul {} este impar.", num);
                }
            } else {
                println!("Nu este numar");
            }
        } else {
            println!("Nu exista numar de verificat");
        }
    }
}
struct PingCommand;
impl ImCom for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, argumente: &[&str]) {
        println!("pong!");
    }
}
struct CountCommand;
impl ImCom for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, argumente: &[&str]) {
        println!("Am numarut : {}", argumente.len());
    }
}
struct TimesCommand {
    count: usize,
}
impl ImCom for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, argumente: &[&str]) {
        self.count += 1;
        println!("Comanda executata de {} ori", self.count);
    }
}
struct Terminal {
    comanda: Vec<Box<dyn ImCom>>,
}
impl Terminal {
    fn new() -> Self {
        Terminal {
            comanda: Vec::new(),
        }
    }
    fn register(&mut self, comanda: Box<dyn ImCom>) {
        self.comanda.push(comanda);
    }
    fn run(&mut self) {
        let date = fs::read_to_string("src/1.txt");
        match date {
            Ok(date) => {
                for line in date.lines() {
                    let mut cuvinte: Vec<&str> = line.trim().split_whitespace().collect();
                    if !cuvinte.is_empty() {
                        let input = cuvinte.remove(0);
                        if input == "stop" {
                            println! {"Oprit."};
                            std::process::exit(0);
                        }
                        let mut ok = false;
                        for command in &mut self.comanda {
                            if command.get_name() == input {
                                command.exec(&cuvinte);
                                ok = true;
                                break;
                            }
                        }

                        if !ok {
                            println!("Comanda necunoscuta: {}", input);
                        }
                    }
                }
            }
            Err(err) => {
                println!("Eroare la citire: {}", err);
            }
        }
    }
}
fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(ParitateCommand {}));

    terminal.run();
}
