enum Error {
    NotAscii,
    NotDigit,
    Not16,
    NotLetter,
    NotPrintable,
}
fn to_uppercase(caracter: char) -> Result<char, Error> {
    if (caracter >= 'a' && caracter <= 'z') || (caracter >= 'A' && caracter <= 'Z') {
        return Ok((caracter as u8 - 32) as char);
    }
    return Err(Error::NotLetter);
}

fn to_lowercase(caracter: char) -> Result<char, Error> {
    if (caracter >= 'A' && caracter <= 'Z') || (caracter >= 'a' && caracter <= 'z') {
        return Ok((caracter as u8 + 32) as char);
    }
    return Err(Error::NotLetter);
}
fn print_char(caracter: char) -> Result<char, Error> {
    if caracter.is_ascii() {
        return Ok(caracter);
    }
    return Err(Error::NotPrintable);
}
fn char_to_number(caracter: char) -> Result<u32, Error> {
    if !caracter.is_ascii() {
        return Err(Error::NotAscii);
    }
    if caracter >= '0' && caracter <= '9' {
        return Ok(caracter as u32 - '0' as u32);
    }
    return Err(Error::NotDigit);
}
fn char_to_number_hex(caracter: char) -> Result<char, Error> {
    if !caracter.is_ascii() {
        return Err(Error::NotAscii);
    }
    if (caracter >= '0' && caracter <= '9')
        || (caracter >= 'A' && caracter <= 'Z')
        || (caracter >= 'a' && caracter <= 'z')
    {
        return Ok(caracter);
    }
    return Err(Error::Not16);
}
fn print_error(error: Error) {
    match error {
        Error::NotAscii => println!("Eroare : caracterul nu este ascii"),
        Error::NotDigit => println!("Eroare: caracterul nu este o cifra"),
        Error::Not16 => println!("Eroare: caracterul nu este in baza 16"),
        Error::NotLetter => println!("Eroare: caracterul nu este o litera"),
        Error::NotPrintable => println!("Eroare: caracterul nu poate sa fie afisat"),
    }
}
fn main() {
    let caracter_mare = 'A';
    let caracter_mic = 'a';
    let caracter_rau = '.';
    match to_lowercase(caracter_mare) {
        Ok(caracter) => println!("Litera mica este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    match to_lowercase(caracter_rau) {
        Ok(caracter) => println!("Litera mica este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    match to_uppercase(caracter_mic) {
        Ok(caracter) => println!("Litera mare este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    match to_uppercase(caracter_rau) {
        Ok(caracter) => println!("Litera mare este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    let caracter_printabil = 'o';
    let caracter_neprintabil = 'Ă';
    match print_char(caracter_printabil) {
        Ok(caracter) => println!("Caracterul este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    match print_char(caracter_neprintabil) {
        Ok(caracter) => println!("Caracterul este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    let numar = '6';
    let nu_numar = 'x';
    match char_to_number(numar) {
        Ok(caracter) => println!("Numarul este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    match char_to_number(nu_numar) {
        Ok(caracter) => println!("Numarul este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    let nr_baza16 = 'B';
    let nu_nr_baza16 = 'J';
    match char_to_number_hex(nr_baza16) {
        Ok(caracter) => println!("Numarul in baza 16 este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
    match char_to_number_hex(nu_nr_baza16) {
        Ok(caracter) => println!("Numarul in baza 16 este {}", caracter),
        Err(eroare) => print_error(eroare),
    }
}
