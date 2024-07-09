use std::fs;

use walkdir::WalkDir;

#[derive(Debug)]
pub struct Configuration {
    pub file_extensions: Vec<String>,
    pub file_names: Vec<String>,
    pub dirs: Vec<String>,
}

impl Configuration { 
    pub fn new(args: Vec<String>) -> Self {
        let mut config = Self {
            file_extensions: vec![],
            file_names: vec![],
            dirs: vec![],
        };

        config.parse_args(args);
        config.verify_files_exist();
        config.find_file_in_dir_with_ext();

        config
    }

    fn parse_args(&mut self, args: Vec<String>) {
        for arg in args {
            println!("Args = {arg}");
            let chars: Vec<char> = arg.chars().collect();
            // Take care of wrong arguments
            if chars[0] != '-' || chars[2] != '=' || !['d', 'e', 'f'].contains(&chars[1]) {
                eprintln!(
                    "ERROR: '{arg}' is not a valid configuration argument. Usage:\n\
                    Specify one or more files: `-f=/path/to/file.rs,/path/to/otherFile.rs`.\n\
                    Specify a directory: `-d=/dir/of/code1,/dir/of/code2` `-e=js,html`."
                );
                panic!("Invalid configuration argument");
            }

            let right_hand_side = chars[3..]
                .iter()
                .collect::<String>()
                .split(",")
                .map(|s| s.into())
                .collect::<Vec<String>>();

            match chars[1] {
                'd' => self.dirs.extend(right_hand_side),
                'e' => self.file_extensions.extend(right_hand_side),
                'f' => self.file_names.extend(right_hand_side),
                _ => continue
            }
        }
        println!("Config after parsing arguments: {:?}", self)
    }

    fn verify_files_exist(&mut self) {
        self.file_names = self.file_names
            .iter()
            .filter(|p| {
                if !fs::metadata(p).is_ok() {
                    eprintln!("WARNING: File '{}' does not exist", p);
                    return false;
                }
                true
            })
            .map(|s| s.into())
            .collect::<Vec<_>>();
    }

    fn find_file_in_dir_with_ext(&mut self) {
        for root_dir in &self.dirs {
            for entry in WalkDir::new(root_dir) {
                let entry = match entry {
                    Ok(dir_entry) => dir_entry,
                    Err(err) => {
                        eprintln!("WARNING: Failed to find {}, error: {:?}", root_dir, err);
                        continue;
                    },
                };

                println!("Current entry: {:?}", entry);

                if let Some(ext) = entry.path().extension() {
                    if !self.file_extensions.contains(&ext.to_str().unwrap().into()) {
                        continue;
                    }
                    if let Some(cur_file_name) = entry.path().to_str() {
                        self.file_names.push(cur_file_name.into())
                    }
                }
            }
        }
         println!("Config after walkdir: {:?}", self);
    }
}