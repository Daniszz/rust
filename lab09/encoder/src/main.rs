use base64;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[clap(author, version = env!("CARGO_PKG_VERSION"), about)]
struct Cli {
    #[clap(long, short, value_parser)]
    intrare: Option<String>,

    #[clap(long, short, value_parser)]
    iesire: Option<String>,
}

fn main() {
    afisare();
    let cli = Cli::parse();

    match (cli.intrare, cli.iesire) {
        (Some(intrare), Some(iesire)) => {
            if let Err(e) = procesare(intrare, iesire) {
                eprintln!("Eroare: {}", e);
            }
        }
        _ => {
            let intrare = citire();
            println!("{}", base64::encode(intrare));
        }
    }
}

fn afisare() {
    let versiune = env!("CARGO_PKG_VERSION");
    let os = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "un sistem de operare necunoscut"
    };

    println!("Encoder, versiune {}, construit pentru {}", versiune, os);
}

fn procesare(intrare_cale: String, iesire_cale: String) -> io::Result<()> {
    let mut fisier_intrare = File::open(intrare_cale)?;
    let mut continut = Vec::new();
    fisier_intrare.read_to_end(&mut continut)?;
    let codat = base64::encode(&continut);
    let mut fisier_iesire = File::create(iesire_cale)?;
    fisier_iesire.write_all(codat.as_bytes())?;
    Ok(())
}

fn citire() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Eroare la citirea de la stdin");
    buffer.trim_end().to_string()
}
