use std::process;
use colored::*;
pub struct DataMM {
	pub min_0: f64,
	pub max_0: f64,
	pub min_1: f64,
	pub max_1: f64,
}
pub fn normalize_elem(el: f64, min: f64, max: f64) -> f64 {
    (el - min) / (max - min)
}

pub fn denormalize_elem(el: f64, min: f64, max: f64) -> f64 {
    (el * (max - min)) + min
}
pub fn error_invalid_key(section: &str, key: &str) {
	println!("{}: invalid key [{}] in section [{}] in .ini file", "error".red().bold(), key, section);
	process::exit(1);
}