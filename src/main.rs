use deployer::Deployer;

fn main() {
	let attr = std::env::args().skip(1).next();
	if attr.is_none() {
		eprintln!("No config file specified");
		std::process::exit(1);
	}
	let mut deployer = Deployer::new();
	deployer.configure(attr.unwrap().as_str());
	deployer.deploy();
}
