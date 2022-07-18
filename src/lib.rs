use std::path::PathBuf;
use std::env::{current_dir, current_exe};
use std::fs;

use fs_extra::dir::CopyOptions;

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
	pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
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
					return Err("The working directory is invalid!");
				}
			};

			let filename = if filename.is_absolute() {
				filename
			} else {
				dir.join(filename)
			};

			Ok(Config {action, filename})
		} else {
			Err("No command given!")
		}
	}
}

pub fn clipboard() -> Result<PathBuf, &'static str> {
	let clipboard_filename = "clipboard";

	let mut dir = match current_exe() {
		Ok(exe) => exe,
		Err(_) => {
			return Err("Couldn't find the clipd executable!");
		}
	};

	dir.pop();

	let clipboard = dir.join(clipboard_filename);

	Ok(clipboard)
}

pub fn run(config: Config) -> Result<(), &'static str> {
	match config.action {
		Action::Copy | Action::Cut => {
			if !config.filename.exists() {
				return Err("The given file does not exist!");
			}

			let clipboard = clipboard()?;

			let contents = format!("{}\n{}", match config.action {
				Action::Copy => "copy",
				Action::Cut => "cut",
				_ => ""
			}, config.filename.display());

			if fs::write(clipboard, contents).is_err() {
				eprintln!("Couldn't write to the clipboard!");
			}
		},
		Action::Paste => {
			if !config.filename.exists() {
				if fs::create_dir_all(&config.filename).is_err() {
					return Err("Couldn't create the target directory.");
				}
			}

			let lines = match fs::read_to_string(clipboard()?) {
				Ok(contents) => contents,
				Err(_) => {
					return Err("Couldn't read from clipboard!");
				}
			};

			let mut lines = lines.lines();

			let delete_source = match lines.next() {
				Some(mode) => {
					match mode {
						"copy" => false,
						"cut" => true,
						_ => {
							return Err("Invalid data stored in clipboard!");
						}
					}
				},
				None => {
					return Err("Clipboard is empty!");
				}
			};

			let source = match lines.next() {
				Some(line) => PathBuf::from(line),
				None => {
					return Err("Incomplete data stored in clipboard!");
				}
			};

			// let items = if source.is_dir() {
			// 	if let Ok(dir_contents) = fs::read_dir(source) {

			// 		let mut sources = Vec::new();

			// 		for item in dir_contents {
			// 			if let Ok(item) = item {
			// 				sources.push(item.path());
			// 			} else {
			// 				return Err("An intermittent IO error occured!");
			// 			}
			// 		};

			// 		sources
			// 	} else {
			// 		return Err("Couldn't copy the folder referenced by the clipboard. Make sure the folder has not been deleted nor moved.");
			// 	}
			// } else {
			// 	vec![source]
			// };
			
			// TODO: add a progress bar
			if delete_source {
				if fs_extra::move_items(&[source], config.filename, &CopyOptions::new()).is_err() {
					// TODO: Improve error messages, especially here
					return Err("Couldn't move the items!");
				}

				if fs::write(clipboard()?, "").is_err() {
					return Err("The clipboard could not be cleared.");
				}
			} else {
				if fs_extra::copy_items(&[source], config.filename, &CopyOptions::new()).is_err() {
					return Err("Couldn't copy the items!");
				}
			}
		}
	}

	Ok(())
}
