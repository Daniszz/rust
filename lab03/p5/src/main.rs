#[derive(Debug)]

enum Error {
    NotDigit,
    NotAscii,
}
fn verificare2(caracter: char) -> Result<u16, Error> {
    if !caracter.is_ascii() {
        return Err(Error::NotAscii);
    }
    if caracter >= '0' && caracter <= '9' {
        return Ok(caracter as u16 - '0' as u16);
    }
    return Err(Error::NotDigit);
}
fn verificare1(caracter_sute: char, caracter_zeci: char, caracter_unitati: char) -> Option<u8> {
    let  numar: u16;
    let sute = match verificare2(caracter_sute) {
        Ok(sute) => sute,
        Err(err) => {
            println!("Eroare la sute: {:?}", err);
            return None;
        }
    };
    let zeci = match verificare2(caracter_zeci) {
        Ok(zeci) => zeci,
        Err(err) => {
            println!("Eroare la zeci: {:?}", err);
            return None;
        }
    };
    let unitati = match verificare2(caracter_unitati) {
        Ok(unitati) => unitati,
        Err(err) => {
            println!("Eroare la unitati: {:?}", err);
            return None;
        }
    };
    numar = sute * 100 + zeci * 10 + unitati;
    if numar > std::u8::MAX as u16 {
        return None;
    } else {
        return Some(numar as u8);
    }
}
fn main() {
    let caracter_sute = '2';
    let caracter_zeci = '5';
    let caracter_unitati = '3';
    match verificare1(caracter_sute, caracter_zeci, caracter_unitati) {
        Some(rezultat) => println!("Numarul este {}", rezultat),
        None => println!("None"),
    }
}
