use std;
use std::io::Write;
use yaml_rust::{Yaml, YamlLoader};
use std::collections::HashMap;

pub fn parse_from_str(data: &str, keys: Vec<String>, options: &mut HashMap<String, Option<String>>, separater: &mut String) -> bool {
	match YamlLoader::load_from_str(data) {
		Ok(docs) => {
			parse(docs, keys, options, separater);
			true
		}, 
		Err(e) => {
			writeln!(&mut std::io::stderr(), "{}", e);
			false
		}
	}
}

fn parse(docs: Vec<Yaml>, keys: Vec<String>, options: &mut HashMap<String, Option<String>>, separater: &mut String) {
	let docs_num = docs.len();
	for key in keys {
		for i in 0..docs_num {
			let node = &docs[i][key.as_str()];
			if node.is_badvalue() {
				continue
			}
			parse_separater(&node["separater"], separater);
			parse_options(&node["options"], options);
		}
	}
}

fn parse_separater(node: &Yaml, separater: &mut String) {
	if node.is_badvalue() {
		return
	}
	let s = match node.as_str() {
		Some(s) => s,
		None => return
	};
	separater.clear();
	separater.push_str(s);
}

fn parse_options(node: &Yaml, options: &mut HashMap<String, Option<String>>) {
	if node.is_badvalue() {
		return
	}
	let hash = match node.as_hash() {
		Some(hash) => hash,
		None => return
	};
	for (k, v) in hash {
		let key = match k.as_str() {
			Some(s) => String::from(s),
			None => {
				match k.as_i64() {
					Some(n) => n.to_string(),
					None => continue
				}
			}
		};
		let value = match v.as_str() {
			Some(s) => Some(String::from(s)),
			None => {
				match v.as_i64() {
					Some(n) => Some(n.to_string()),
					None => None
				}
			}
		};
		options.insert(key, value);
	}
}