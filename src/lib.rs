use std::path::PathBuf;
use std::env::current_dir;

#[derive(PartialEq, Eq, Debug)]
pub enum Action {
	Copy,
	Cut,
	Paste
}

pub struct Config {
	pub action: Action,
	pub filename: PathBuf
}

impl Config {
	pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
		args.next();

		let action = match args.next() {
			Some(command) => {
				match command.as_str() {
					"copy" => Action::Copy,
					"cut" => Action::Cut,
					"paste" => Action::Paste,
					_ => return Err("Invalid command!")
				}
			},
			None => {
				return Err("No command given!");
			}
		};

		if let Some(filename) = args.next() {
			let filename = PathBuf::from(filename); 

			let dir = match current_dir() {
				Ok(value) => value,
				Err(_) => {
					return Err("The current directory is invalid!");
				}
			};

			let filename = if filename.is_absolute() {
				filename
			} else {
				dir.join(filename)
			};

			if !filename.exists() {
				return Err("The given file does not exist!");
			}

			Ok(Config {action, filename})
		} else {
			Err("No command given!")
		}
	}
}

pub fn run(config: Config) {
	println!("{}", config.filename.display());
}
