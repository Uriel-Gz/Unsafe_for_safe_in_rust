use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use syn::{visit_mut::VisitMut, visit::Visit, File, Expr, ExprUnsafe};
use syn::spanned::Spanned;
use proc_macro2::{TokenTree, TokenStream, Span};
use quote::ToTokens;
use serde::Serialize;
use serde_json::{self, to_string};
mod pattern_detector;
use pattern_detector::PatternDetector;
use std::collections::HashMap;
use walkdir::WalkDir;
mod extractor;
mod modifier;
mod config;
mod suggestions;
use suggestions::generate_suggestions;
use modifier::replace_unsafe_code;
use extractor::{process_file, extract_unsafe_blocks};

/// Display the main menu and return user's choice
pub fn display_menu() -> String {
    println!("\n{}Seleccione una opción:{}", "\x1b[0m", "\x1b[0m\n");
    println!("{}(1) {}\x1b[0mExtraer código unsafe", "\x1b[33m", "\x1b[0m");
    println!("{}(2) {}\x1b[0mReemplazar código unsafe", "\x1b[93m", "\x1b[0m");
    println!("{}(3) {}\x1b[0mObtener sugerencias de código seguro", "\x1b[91m", "\x1b[0m");
    println!("{}(4) {}\x1b[0mSalir\n", "\x1b[34m", "\x1b[0m");
    
    print!("Ingrese su elección: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Display the welcome banner
pub fn show_init() {
    // Clear terminal
    print!("\x1B[2J\x1B[1;1H");

    println!("{}\x1b[94m             ================================================== \x1b[0m", "");
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
pub fn get_file_path(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Main event loop
fn main() -> Result<()> {
    show_init();
    
    let out_dir = PathBuf::from("result");
    let report = PathBuf::from("report");
    let out_dir_ch = PathBuf::from("result_changed");
    
    loop {
        let choice = display_menu();
        
        match choice.as_str() {
            "1" => {
                fs::create_dir_all(&out_dir)?;
                let path = get_file_path("Ingrese la ruta del archivo Rust: ");
                if Path::new(&path).exists() {
                    println!("\n\x1b[92m✓ Extrayendo código unsafe...\x1b[0m");
                    let path_to = PathBuf::from(path.clone());
                    extractor::extract_unsafe_blocks(&path_to, &out_dir)?;
                    println!("\x1b[92m✓ Extracción completada\x1b[0m");
                } else {
                    println!("\x1b[91m✗ El archivo no existe\x1b[0m");
                }
            }
            "2" => {
                fs::create_dir_all(&report)?;
                fs::create_dir_all(&out_dir_ch)?;
                let path = get_file_path("Ingrese la ruta del archivo Rust: ");
                if Path::new(&path).exists() {
                    println!("\n\x1b[92m✓ Reemplazando código unsafe...\x1b[0m");
                    let path_to = PathBuf::from(path.clone());
                    modifier::replace_unsafe_code(&path_to, &out_dir_ch, Some(&report))?;
                    println!("\x1b[92m✓ Reemplazo completado\x1b[0m");
                } else {
                    println!("\x1b[91m✗ El archivo no existe\x1b[0m");
                }
            }
            "3" => {
                let path = get_file_path("Ingrese la ruta del archivo o directorio Rust: ");
                let path_buf = PathBuf::from(path.clone());
                if path_buf.exists() {
                    println!("\n\x1b[92m✓ Analizando código para generar sugerencias...\x1b[0m");
                    match generate_suggestions(&path_buf) {
                        Ok(generator) => generator.display_suggestions(),
                        Err(e) => println!("\x1b[91m✗ Error: {}\x1b[0m", e),
                    }
                } else {
                    println!("\x1b[91m✗ El archivo o directorio no existe\x1b[0m");
                }
            }
            "4" => {
                println!("\nSaliendo del programa.");
                break;
            }
            _ => {
                println!("\x1b[91mOpción inválida. Por favor, intente de nuevo.\x1b[0m");
            }
        }
    }

    Ok(())
}
