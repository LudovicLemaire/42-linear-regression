use ini::Ini;
use std::process;
use std::io;
use std::io::Write;
use colored::*;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut need_y = false;
	
	if args.len() > 1 && args[1] == "_find_X" {
		need_y = true;
	}

	let mut theta_0:f32 = 0.0;
	let mut theta_1:f32 = 0.0;

	let file = Ini::load_from_file("./data/theta.ini").unwrap();
	for (_section, prop) in file.iter() {
		for (k, v) in prop.iter() {
			match k {
				"theta_0" => {
					theta_0 = v.parse::<f32>().unwrap_or_else(|e| {
						println!("{}: {}: {}", "error".red().bold(), k, e);
						process::exit(1);
					})
				}
				"theta_1" => {
					theta_1 = v.parse::<f32>().unwrap_or_else(|e| {
						println!("{}: {}: {}", "error".red().bold(), k, e);
						process::exit(1);
					})
				}
				key => {
					println!("{}: invalid key [{}] in .ini file", "error".red().bold(), key);
					process::exit(1);
				}
			}
		}
	}

	if theta_0 == 0.0 && theta_1 == 0.0 {
		println!("{}: {}", "warning".yellow().bold(), "it looks like thetas aren't generated, did you forgot to train the model ?");
		process::exit(1);
	}

	println!("{}: {}", "theta_0".bright_purple(), theta_0);
	println!("{}: {}\n", "theta_1".bright_purple(), theta_1);
	
	let mut user_input = String::new();
	print!("{}: ", "Enter mileage".bright_cyan());
	io::stdout().flush().unwrap(); // force the print to show up

	io::stdin().read_line(&mut user_input).unwrap_or_else(|e| {
        println!("{}: {}", "error".red().bold(), e);
        process::exit(1);
    });
	
	let mileage = user_input.trim().parse::<f32>().unwrap_or_else(|e| {
		println!("{}: {}", "error".red().bold(), e);
		process::exit(1);
	});

	if need_y == true {
		println!("{}: {}km", "Expected mileage".bright_blue(), (mileage - theta_0) / theta_1);
	} else {
		println!("{}: {}", "Expected price".bright_blue(), theta_0 + theta_1 * mileage);
	}
}
