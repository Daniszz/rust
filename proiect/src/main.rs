use std::fs;
use std::io;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

fn copiere_director(cale_sursa: &str, cale_destinatie: &str) -> io::Result<()> {
    if cale_sursa == cale_destinatie {
        eprintln!(
            "cp: cannot copy a directory, '{}' , into itself, '{}",
            cale_sursa, cale_destinatie
        );
        return Err(io::Error::new(ErrorKind::Other, "They are the same"));
    }
    if let Ok(sursa_metadata) = fs::metadata(cale_sursa) {
        if !sursa_metadata.is_dir() {
            eprintln!(
                "cp: cannot stat '{}': No such file or directory",
                cale_sursa
            );
            return Err(io::Error::new(ErrorKind::Other, "Not directory"));
        }
    } else {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "Source directory does not exist",
        ));
    }

    let cale_sursa1 = Path::new(cale_sursa);
    let cale_destinatie1 = Path::new(cale_destinatie);

    let destinatie_director = if cale_destinatie1.is_dir() {
        cale_destinatie1.join(cale_sursa1.file_name().unwrap())
    } else {
        fs::create_dir(cale_destinatie1)?;

        cale_destinatie1.to_path_buf()
    };

    if !destinatie_director.exists() {
        fs::create_dir(&destinatie_director)?;
    }

    if cale_sursa1.is_dir() {
        for entry in fs::read_dir(cale_sursa1)? {
            let entry = entry?;
            let entry_path = entry.path();
            println!("{}", entry_path.to_string_lossy());
            if entry_path.is_dir() {
                copiere_director(
                    &entry_path.to_string_lossy(),
                    &destinatie_director.to_string_lossy(),
                )?;
            } else {
                let destinatie_fisier = destinatie_director.join(entry_path.file_name().unwrap());

                  copiere_fisier(
                    &entry_path.to_string_lossy(),
                    &destinatie_fisier.to_string_lossy(),
                ).unwrap_or_default();
            }
        }
    }

    Ok(())
}

fn copiere_fisier(cale_sursa: &str, cale_destinatie: &str) -> io::Result<()> {
    #[cfg(windows)]
    let path_separator = '\\';
    if cale_sursa == cale_destinatie {
        eprintln!(
            "cp: '{}' and '{}' are the same file",
            cale_sursa, cale_destinatie
        );
        return Err(io::Error::new(ErrorKind::Other, "They are the same"));
    }
    if let (Ok(sursa_metadata), Ok(destinatie_metadata)) =
        (fs::metadata(cale_sursa), fs::metadata(cale_destinatie))
    {
        if sursa_metadata.is_dir() && destinatie_metadata.is_dir() {
            eprintln!("cp: option -r was not specified. This command only works to copy a directory tree as a regular file.");
            return Err(io::Error::new(
                ErrorKind::Other,
                "Option -r was not specified",
            ));
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

    let sursa_contents = match fs::read(cale_sursa) {
        Ok(contents) => contents,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => eprintln!(
                    "cp: cannot stat '{}': No such file or directory",
                    cale_sursa
                ),
                ErrorKind::PermissionDenied => eprintln!(
                    "cp: cannot read '{}' for copying: Permission denied",
                    cale_sursa
                ),
                _ => eprintln!("cp: {}", e),
            }
            return Err(e);
        }
    };
    match fs::write(&modified_destinatie, sursa_contents) {
        Ok(_) => Ok(()),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => eprintln!(
                    "cp: cannot stat '{}': No such file or directory",
                    cale_destinatie
                ),
                ErrorKind::PermissionDenied => eprintln!(
                    "cp: cannot read '{}' for copying: Permission denied",
                    cale_destinatie
                ),
                _ => eprintln!("cp: {}", e),
            }
            Err(e)
        }
    }
}

fn stergere_fisier(director: &[&str]) -> io::Result<()> {
    for &fisier in director {
        match fs::remove_file(fisier) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("rm: cannot remove '{}': No such file or directory", fisier)
                }
                ErrorKind::PermissionDenied => eprintln!(
                    "cp: cannot read '{}' for copying: Permission denied",
                    fisier
                ),
                _ => eprintln!("Failed to delete file '{}': {}", fisier, e),
            },
        }
    }

    Ok(())
}
fn stergere_director(directoare: &[&str]) -> io::Result<()> {
    for &director in directoare {
        match fs::remove_dir_all(director) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => eprintln!(
                    "rm: cannot remove '{}': No such file or directory",
                    director
                ),
                ErrorKind::PermissionDenied => eprintln!(
                    "cp: cannot read '{}' for copying: Permission denied",
                    director
                ),
                _ => eprintln!("Failed to delete file '{}': {}", director, e),
            },
        }
    }

    Ok(())
}
fn mutare(fisiere: &[&str]) -> io::Result<()> {
    let destinatie = fisiere.last().unwrap();

    for fisier in fisiere.iter().take(fisiere.len() - 1) {
        match fs::metadata(fisier) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    println!("da");

                     copiere_director(fisier, destinatie).unwrap_or_default();
                      stergere_director(&[fisier]).unwrap_or_default();
                } else if metadata.is_file() {
                    println!("Nu");

                    copiere_fisier(fisier, destinatie)?;
                    fs::remove_file(fisier)?;
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("mv: cannot move '{}': No such file or directory", fisier)
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("mv: cannot read '{}' for moving: Permission denied", fisier)
                }
                _ => eprintln!("Failed to delete file '{}': {}", fisier, e),
            },
        }
    }

    Ok(())
}
fn listare_key(input: &str) -> io::Result<()> {
    let (predef, subkey) = input.split_once('\\').unwrap_or((input, ""));

    let hkey = match predef.to_uppercase().as_str() {
        "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKEY_USERS" => HKEY_USERS,
        "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
        "HKEY_PERFORMANCE_DATA" => HKEY_PERFORMANCE_DATA,
        "HKEY_PERFORMANCE_TEXT" => HKEY_PERFORMANCE_TEXT,
        "HKEY_PERFORMANCE_NLSTEXT" => HKEY_PERFORMANCE_NLSTEXT,

        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid predefined key: {}", predef),
            ));
        }
    };

    let key = RegKey::predef(hkey).open_subkey(subkey)?;

    for subkey in key.enum_keys() {
        print!("{}\\", input);

        println!("{}", subkey?);
    }

    Ok(())
}
fn creare_key(input: &str) -> io::Result<()> {
    println!("Creating key: {}", input);

    let (predef, subkey) = input.split_once('\\').unwrap_or((input, ""));

    let hkey = match predef.to_uppercase().as_str() {
        "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKEY_USERS" => HKEY_USERS,
        "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
        "HKEY_PERFORMANCE_DATA" => HKEY_PERFORMANCE_DATA,
        "HKEY_PERFORMANCE_TEXT" => HKEY_PERFORMANCE_TEXT,
        "HKEY_PERFORMANCE_NLSTEXT" => HKEY_PERFORMANCE_NLSTEXT,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid predefined key: {}", predef),
            ));
        }
    };

    if let Err(err) = RegKey::predef(hkey).create_subkey(subkey) {
        eprintln!("Failed to create subkey: {}", err);
        
    }
    println!("Key '{}' created successfully.", input);

    Ok(())
}
fn modificare_key(input: &str, nume: &str, data: &str) -> io::Result<()> {
    let (predef, subkey) = input.split_once('\\').unwrap_or((input, ""));

    let hkey = match predef.to_uppercase().as_str() {
        "HKCR" | "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKCU" | "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKLM" | "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKU" | "HKEY_USERS" => HKEY_USERS,
        "HKCC" | "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
        "HKPD" | "HKEY_PERFORMANCE_DATA" => HKEY_PERFORMANCE_DATA,
        "HKPT" | "HKEY_PERFORMANCE_TEXT" => HKEY_PERFORMANCE_TEXT,
        "HKPN" | "HKEY_PERFORMANCE_NLSTEXT" => HKEY_PERFORMANCE_NLSTEXT,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid predefined key: {}", predef),
            ));
        }
    };

    let key = RegKey::predef(hkey);
    let path = Path::new(subkey);
    let (key, disp) = key.create_subkey(path)?;

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }

    key.set_value(nume, &data)?;

    println!("Registry value added successfully.");

    Ok(())
}
fn stergere_key(input: &str) -> io::Result<()> {
    println!("Da");
    let (predef, subkey) = input.split_once('\\').unwrap_or((input, ""));

    let hkey = match predef.to_uppercase().as_str() {
        "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKEY_USERS" => HKEY_USERS,
        "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
        "HKEY_PERFORMANCE_DATA" => HKEY_PERFORMANCE_DATA,
        "HKEY_PERFORMANCE_TEXT" => HKEY_PERFORMANCE_TEXT,
        "HKEY_PERFORMANCE_NLSTEXT" => HKEY_PERFORMANCE_NLSTEXT,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid predefined key: {}", predef),
            ));
        }
    };

    if let Err(err) = RegKey::predef(hkey).delete_subkey(subkey) {
        eprintln!("Failed to delete subkey: {}", err);
        
    }
    println!("Key '{}' deleted successfully.", input);

    Ok(())
}
fn listare_procese() {
    let iesire = Command::new("tasklist")
        .output()
        .expect("Eroare la executie");
    if iesire.status.success() {
        let text = String::from_utf8_lossy(&iesire.stdout);
        println!("{}", text);
    } else {
        eprintln!("Error listing processes");
    }
}
fn kill_proces(proces: &str) {
    let iesire = Command::new("taskkill")
        .arg("/F")
        .arg(match proces.parse::<u32>() {
            Ok(_) => "/PID",
            Err(_) => "/IM",
        })
        .arg(proces)
        .output()
        .expect("Failed to execute taskkill command");
    if iesire.status.success() {
        println!(
            "Successfully killed processes with '{}': {}",
            if proces.parse::<u32>().is_ok() {
                "PID"
            } else {
                "name"
            },
            proces
        );
    } else {
        eprintln!(
            "Error killing processes with '{}': {}",
            if proces.parse::<u32>().is_ok() {
                "PID"
            } else {
                "name"
            },
            proces
        );
    }
}
fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Eroare la citirea liniei");
        let argument: Vec<&str> = input.split_whitespace().collect();
        match argument[0] {
            "cp" => {
                if argument.len() == 3 {
                    copiere_fisier(argument[1], argument[2]).unwrap_or_default();
                } else if argument.len() == 4 && argument[1] == "-r" {
                    copiere_director(argument[2], argument[3]).unwrap_or_default();
                } else {
                    println!("cp: missing file operand.\n Try 'cp --help' for more information");
                }
            }
            "rm" => {
                if argument.len() > 1 && argument[1] != "-r" {
                    stergere_fisier(&argument[1..]).unwrap_or_default();
                } else if argument.len() > 1 && argument[1] == "-r" {
                    stergere_director(&argument[2..]).unwrap_or_default();
                } else {
                    println!("rm: missing file operand.\n Try 'rm --help' for more information");
                }
            }
            "mv" => {
                if argument.len() > 2 {
                    mutare(&&argument[1..]).unwrap_or_default();
                } else {
                    println!("mv: missing file operand.\n Try 'mv --help' for more information");
                }
            }
            "reg" => {
                if argument[1] == "query" && argument.len() == 3 {
                    listare_key(argument[2]).unwrap_or_default();
                } else if argument[1] == "add" && argument.len() == 3 {
                    creare_key(argument[2]).unwrap_or_default();
                } else if argument[1] == "add" && argument[3] == "/v" && argument[5] == "/d" {
                    modificare_key(argument[2], argument[4], argument[6]).unwrap_or_default();
                } else if argument[1] == "delete" && argument.len() == 3 {
                    stergere_key(argument[2]).unwrap_or_default();
                } else {
                    println!("Command not recognized");
                }
            }
            "ps" => {
                if argument.len() == 1 {
                    listare_procese();
                }
            }
            "pkill" => {
                if argument.len() == 2 {
                    println!("da");
                    kill_proces(argument[1]);
                } else {
                    println!(
                        "pkill: missing file operand.\n Try 'pkill --help' for more information"
                    );
                }
            }
            "kill" => {
                if argument.len() == 2 {
                    kill_proces(argument[1]);
                } else {
                    println!(
                        "kill: missing file operand.\n Try 'kill --help' for more information"
                    );
                }
            }
            _ => {
                println!("syntax error near unexpected token '{}'", argument[0]);
            }
        }
    }
}
