use std::env::args;
use std::process::exit;
use deployer::Deployer;

fn main() {
    println!("Hello, world!");
	let config_file = args().skip(1).next();
	if config_file.is_none() {
		eprintln!("No config file specified");
		exit(1);
	}
	println!("{}", config_file.unwrap());
	// Deployer::new();
}
