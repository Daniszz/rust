use thiserror::Error;
#[derive(Error, Debug)]

enum Error {
    #[error("{0} is not a ascii character")]
    NotAscii(char),
    #[error("{0} is not a digit")]
    NotDigit(char),
    #[error("{0} is not a number in base 16")]
    Not16(char),
    #[error("{0} is not a letter")]
    NotLetter(char),
    #[error("{0} is not printable")]
    NotPrintable(char),
}
fn to_uppercase(caracter: char) -> Result<char, Error> {
    if (caracter >= 'a' && caracter <= 'z') || (caracter >= 'A' && caracter <= 'Z') {
        return Ok((caracter as u8 - 32) as char);
    }
    return Err(Error::NotLetter(caracter));
}

fn to_lowercase(caracter: char) -> Result<char, Error> {
    if (caracter >= 'A' && caracter <= 'Z') || (caracter >= 'a' && caracter <= 'z') {
        return Ok((caracter as u8 + 32) as char);
    }
    return Err(Error::NotLetter(caracter));
}
fn print_char(caracter: char) -> Result<char, Error> {
    if caracter.is_ascii() {
        return Ok(caracter);
    }
    return Err(Error::NotPrintable(caracter));
}
fn char_to_number(caracter: char) -> Result<u32, Error> {
    if !caracter.is_ascii() {
        return Err(Error::NotAscii(caracter));
    }
    if caracter >= '0' && caracter <= '9' {
        return Ok(caracter as u32 - '0' as u32);
    }
    return Err(Error::NotDigit(caracter));
}
fn char_to_number_hex(caracter: char) -> Result<char, Error> {
    if !caracter.is_ascii() {
        return Err(Error::NotAscii(caracter));
    }
    if (caracter >= '0' && caracter <= '9')
        || (caracter >= 'A' && caracter <= 'E')
        || (caracter >= 'a' && caracter <= 'e')
    {
        return Ok(caracter);
    }
    return Err(Error::Not16(caracter));
}
fn print_error(error: Error) {
    println!("err:{}", error);
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
