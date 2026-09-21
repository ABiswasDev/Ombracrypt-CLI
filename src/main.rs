use clap::{Parser, Subcommand};
use colored::*;
use inquire::{Password, PasswordDisplayMode};
use std::process;

mod crypto;
mod deception;
mod pipeline;

#[derive(Parser)]
#[command(name = "ombracrypt")]
#[command(version = "0.4.3")]
#[command(about = "The zero-trust quantum vault, built for the terminal.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file or directory into a Quantum-Safe Vault (.obv)
    Encrypt {
        /// Path to the target file or directory
        #[arg(short, long)]
        target: String,

        /// Symmetric cipher: 'aes256gcm' or 'xchacha20'
        #[arg(short, long, default_value = "aes256gcm")]
        cipher: String,

        /// KEM Profile: 'standard' or 'cypherpunk'
        #[arg(short, long, default_value = "cypherpunk")]
        kem: String,
    },
    /// Decrypt a Quantum-Safe Vault (.obv) using your Ombracrypt Key (.obk)
    Decrypt {
        /// Path to the encrypted vault (.obv)
        #[arg(short, long)]
        target: String,

        /// Path to the Ombracrypt Key (.obk)
        #[arg(short, long)]
        key: String,
    },
}

fn print_header(mode: &str, target: &str) {
    println!("\n{} {}", "::".magenta(), "OMBRACRYPT QUANTUM VAULT".bold().cyan());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_black());
    println!("{} {}", "▶ Operation:".bright_black(), mode.bold().green());
    println!("{} {}", "▶ Target:   ".bright_black(), target.bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n".bright_black());
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt { target, cipher, kem } => {
            print_header("Encrypt (Lock)", target);
            
            let main_pin = loop {
                let pin = Password::new("? Enter Master Password:")
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .without_confirmation()
                    .prompt()
                    .unwrap_or_else(|_| process::exit(1));

                if pin.trim().is_empty() {
                    println!("{}", "✖ ERROR: Master Password cannot be blank!\n".red());
                    continue;
                }

                let confirm = Password::new("? Confirm Master Password:")
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .without_confirmation()
                    .prompt()
                    .unwrap_or_else(|_| process::exit(1));
                
                if pin == confirm {
                    break pin;
                }
                println!("{}", "✖ ERROR: Passwords do not match. Please try again.\n".red());
            };
                
            let deception_passcode = loop {
                let d_pin = Password::new("? Enter Deception Passcode (Press ENTER to skip):")
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .without_confirmation()
                    .prompt()
                    .unwrap_or_else(|_| process::exit(1));

                if d_pin.is_empty() {
                    break d_pin;
                }

                if d_pin == main_pin {
                    println!("{}", "✖ CRITICAL ERROR: Deception Passcode CANNOT be the same as your Master Password!\n".red());
                    continue;
                }

                let confirm = Password::new("? Confirm Deception Passcode:")
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .without_confirmation()
                    .prompt()
                    .unwrap_or_else(|_| process::exit(1));
                
                if d_pin == confirm {
                    break d_pin;
                }
                println!("{}", "✖ ERROR: Deception Passcodes do not match. Please try again.\n".red());
            };

            println!("\n{}", "> Initializing Engine...".bold().yellow());
            match pipeline::encrypt_vault(target, cipher, kem, &main_pin, &deception_passcode) {
                Ok(msg) => println!("\n{} {}\n", "✔".green(), msg.bold().green()),
                Err(e) => {
                    eprintln!("\n{} {}\n", "✖ ERROR:".red(), e.red());
                    process::exit(1);
                }
            }
        }
        Commands::Decrypt { target, key } => {
            print_header("Decrypt (Unlock)", target);
            
            let main_pin = Password::new("? Enter Master Password:")
                .with_display_mode(PasswordDisplayMode::Masked)
                .without_confirmation()
                .prompt()
                .unwrap_or_else(|_| process::exit(1));
            
            println!("\n{}", "> Initializing Engine...".bold().yellow());
            match pipeline::decrypt_vault(target, Some(key), &main_pin, "") {
                Ok(msg) => println!("\n{} {}\n", "✔".green(), msg.bold().green()),
                Err(e) => {
                    eprintln!("\n{} {}\n", "✖ ERROR:".red(), e.red());
                    process::exit(1);
                }
            }
        }
    }
}