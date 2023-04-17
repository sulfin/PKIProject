use std::env::args;
use std::process::exit;
use acr::aci;
use acr::acr::generate_acr;
use acr::acrconfig::AcrConfig;

fn main() {
    let acrconfig = AcrConfig::default();

    if args().len() < 2 {
        print_usage();
        exit(1)
    }

    match args().nth(1).unwrap().as_str() {
        "root" => {
            println!("Generating root certificate");
            generate_acr(&acrconfig).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                exit(1)
            });
        },
        "intermediate" => {
            println!("Signing intermediate certificate with root certificate");
            aci::generate_aci().unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                exit(1)
            });
        },
        _ => {
            println!("Unknown action");
            print_usage();
            exit(1)
        }
    }

}

fn print_usage() {
    println!("Usage: {} <action>", args().nth(0).unwrap());
    println!("Actions:");
    println!("  root : Generate root certificate");
    println!("  intermediate : Sign intermediate certificate");
}