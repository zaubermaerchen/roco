extern crate argparse;
extern crate yaml_rust;

use std::env;
use std::io;
use std::io::{Read, Write};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use argparse::{ArgumentParser, Store, List};
mod yaml_parser;

fn main() {
	let mut path = get_setting_file_default_path();
	let mut keys: Vec<String> = vec![];
	{
		let mut parser = ArgumentParser::new();
		parser.refer(&mut path).add_option(&["-f", "--file"], Store, "setting file path");
		parser.refer(&mut keys).add_argument("target", List, "target key name");
		parser.parse_args_or_exit();
	}
	
	if keys.len() == 0  {
		return;
	}

	// Read data from setting file
	let mut data = String::new();
	match read_data_from_file(path.as_str(), &mut data) {
		Ok(_) => {}, 
		Err(e) => {
			writeln!(&mut std::io::stderr(), "{}", e);
			return
		}
	};
		
	// Parse data & output string
	let mut options: HashMap<String, Option<String>> = HashMap::new();
	let mut separater: String = String::from(" ");
	if yaml_parser::parse_from_str(data.as_str(), keys, &mut options, &mut separater) {
		output_options(options, separater.as_str());
	}
}

fn get_setting_file_default_path() -> String {
	// default path is "~/.roco/setting.yml"
	let path: PathBuf = match env::home_dir() {
		Some(mut path) => {
			path.push(".roco");
			path.push("setting.yml");
			path
		},
		None => PathBuf::from("setting.yml")
	};
	path.as_os_str().to_str().unwrap().to_string()
}

fn read_data_from_file(path: &str, data: &mut String) -> io::Result<()> {
	let path = Path::new(path);
	let mut file = try!(File::open(&path));
	try!(file.read_to_string(data));
	Ok(())
}

fn output_options(options: HashMap<String, Option<String>>, separater: &str) {
	let mut s: String = String::new();
	for (key, value) in options {
		s.push_str(key.as_str());
		if value != None {
			s.push_str(separater);
			s.push_str(value.unwrap().as_str());
		}
		s.push_str(" ");
	}
	println!("{}", s.trim());
}