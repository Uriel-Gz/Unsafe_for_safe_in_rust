use anyhow::Result;
use rmorph::{config, extractor, modifier, suggestions};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Display the main menu and return user's choice
fn display_menu() -> String {
    println!("\n\x1b[0mSeleccione una opción:\x1b[0m\n");
    println!("\x1b[33m(1) \x1b[0mExtraer código unsafe");
    println!("\x1b[93m(2) \x1b[0mReemplazar código unsafe");
    println!("\x1b[91m(3) \x1b[0mObtener sugerencias de código seguro");
    println!("\x1b[34m(4) \x1b[0mSalir\n");

    print!("Ingrese su elección: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Display the welcome banner
fn show_init() {
    // Clear terminal
    print!("\x1B[2J\x1B[1;1H");

    println!("\x1b[94m             ================================================== \x1b[0m");
    println!("\x1b[94m         ====\x1b[91m    #### \x1b[92m #    #                           \x1b[94m       ====           \x1b[0m");
    println!("\x1b[94m     ====\x1b[91m        #   #\x1b[92m ##  ##  ###  #####  ####  #       \x1b[94m          ====       \x1b[0m");
    println!("\x1b[94m ====\x1b[91m            #### \x1b[92m # ## # #   #  #   # #   # ####   \x1b[94m               ====   \x1b[0m");
    println!("\x1b[94m ====\x1b[91m            #   #\x1b[92m #    # #   #  #     ####  #   #   \x1b[94m              ====   \x1b[0m");
    println!("\x1b[94m     ====\x1b[91m        #   #\x1b[92m #    #  ###   #     #     #   #   \x1b[94m          ====       \x1b[0m");
    println!("\x1b[94m         ====\x1b[97m                                           lite \x1b[94m  ====           \x1b[0m");
    println!("\x1b[94m             ================================================== \x1b[0m");
    println!("\n<< \x1b[92mAnalizador\x1b[97m, \x1b[92mextractor \x1b[92my \x1b[92mmodificador \x1b[97mde código unsafe para Rust\x1b[0m >>\n");
}

/// Get file path from user input
fn get_file_path(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Execute the selected option
fn execute_option(op: &str) -> Result<()> {
    let out_dir = PathBuf::from("result");
    let out_dir_ch = PathBuf::from("result_changed");

    let path = get_file_path("Ingrese la ruta del archivo/directorio Rust: ");
    unsafe {
        config::DIR_NAME = path.clone();
    }
    if Path::new(&path).exists() {
        let path_to = PathBuf::from(path.clone());

        match op {
            "1" => {
                fs::create_dir_all(&out_dir)?;
                println!("\n\x1b[92m Extrayendo código unsafe...\x1b[0m\n");
                extractor::extract_unsafe_blocks(&path_to, &out_dir)?;
                println!("\n\x1b[92m Extracción completada\x1b[0m");
            }
            "2" => {
                fs::create_dir_all(&out_dir_ch)?;
                println!("\n\x1b[92m Reemplazando código unsafe...\x1b[0m\n");
                modifier::replace_unsafe_code(&path_to, &out_dir_ch)?;
                println!("\n\x1b[92m Reemplazo completado\x1b[0m");
            }
            "3" => {
                println!("\n\x1b[92m Analizando código para generar sugerencias...\x1b[0m");
                match suggestions::generate_suggestions(&path_to) {
                    Ok(generator) => generator.display_suggestions(),
                    Err(e) => println!("\x1b[91m✗ Error: {}\x1b[0m", e),
                }
            }
            _ => {
                println!("\x1b[91mOpción inválida. Por favor, intente de nuevo.\x1b[0m");
            }
        }
    } else {
        println!("\x1b[91m✗ El archivo no existe\x1b[0m");
    }
    Ok(())
}

fn main() -> Result<()> {
    show_init();

    loop {
        let choice = display_menu();

        match choice.as_str() {
            "4" => {
                println!("\nSaliendo del programa.");
                break;
            }
            _ => {
                let _ = execute_option(&choice);
            }
        }
    }

    Ok(())
}
