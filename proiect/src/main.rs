use std::fs;
use std::io;
use std::io::{ErrorKind, Write};



fn copiere_fisier(cale_sursa: &str,  cale_destinatie: &str) -> io::Result<()>
{
    
    #[cfg(windows)]
    let path_separator = '\\';
    if cale_sursa==cale_destinatie
    {
        eprintln!("cp: '{}' and '{}' are the same file", cale_sursa, cale_destinatie);
    }
    if let (Ok(sursa_metadata), Ok(destinatie_metadata)) = (fs::metadata(cale_sursa), fs::metadata(cale_destinatie)) {
        if sursa_metadata.is_dir() && destinatie_metadata.is_dir() {
            eprintln!("cp: option -r was not specified. This command only works to copy a directory tree as a regular file.");
            return Err(io::Error::new(ErrorKind::Other, "Option -r was not specified"));
        }
    }

    let mut modified_destinatie = cale_destinatie.to_string();

    if let Ok(destinatie_metadata) = fs::metadata(cale_destinatie) {
        if destinatie_metadata.is_dir() {
            let cale_sursa_split: Vec<&str> = cale_sursa.split(path_separator).collect();
            if let Some(cale_sursa_last) = cale_sursa_split.last() {
                modified_destinatie.push(path_separator);
                modified_destinatie.push_str(cale_sursa_last);
            }
        }
    }
        
    
    let sursa_contents = match fs::read(&cale_sursa) {
        Ok(contents) => contents,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => eprintln!("cp: cannot stat '{}': No such file or directory", cale_sursa),
                ErrorKind::PermissionDenied => eprintln!("cp: cannot read '{}' for copying: Permission denied", cale_sursa),
                _ => eprintln!("cp: {}", e),
            }
            return Err(e);
        }
    };
    match fs::write(&modified_destinatie, &sursa_contents) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => eprintln!("cp: cannot stat '{}': No such file or directory", cale_destinatie),
                ErrorKind::PermissionDenied => eprintln!("cp: cannot read '{}' for copying: Permission denied", cale_destinatie),
                _ => eprintln!("cp: {}", e),
            }
            Err(e)
        }
    }
    
}
  
    

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Eroare la citirea liniei");
        let argument : Vec<&str> =input.split_whitespace().collect();
        match argument[0] {
            "cp" =>
            {
                if argument.len()==3
                {
                    let _ = copiere_fisier(argument[1],argument[2]);
                }
                else {
                    println!("mkdir: missing operand");
                }
            }
            _ => {
                println!("Command not recognized");
            }
        
        }
      
       
    }
}
