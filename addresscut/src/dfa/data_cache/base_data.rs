
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::option::Option::{None, Some};
use std::str::FromStr;

pub struct City {
	pub id:usize,
	pub pid:usize,
	pub lvl:i8,
	pub name:String,
	pub names:Vec<String>
}

pub fn all_citys() -> Vec<City> {
	let mut citys:Vec<City> = vec![];
    let path = Path::new("citybasedata.config");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(cs) => println!("{} size:\n{}", display, cs)
    }

	let lines:Vec<&str> = s.lines().collect();
	for line in lines {
		let ss:Vec<&str> = line.split(',').collect();
		let mut _id:usize = 0;
		let mut _pid:usize = 0;
		let mut _lvl:i8 = 0;
		let mut _names:Vec<String> = Vec::new();
		let mut _name:String = "".to_string();
		for i in 0 .. ss.len() {
			match ss.get(i) {
				None => panic!("couldn't be happen!"),
				Some(s) => {
					match i {
						0 => _id = str_to_int::<usize>(s),
						1 => _pid = str_to_int::<usize>(s),
						2 => _lvl = str_to_int::<i8>(s),
						3 => {_name = s.to_string();_names.push(s.to_string());}
						_ => _names.push(s.to_string()),
					}
				}
			}
		}
		citys.push(City{id:_id, pid:_pid, lvl:_lvl, name:_name, names:_names});
	}

	citys
}

fn str_to_int<T: FromStr>(s: &str) -> T {
	match T::from_str(s) {
		Err(_) => panic!("couldn't convert {}", s),
		Ok(_i) => _i,
	}
}