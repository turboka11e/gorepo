use std::env;
use std::fs;
use std::path::PathBuf;

fn read_directory(dir_path: PathBuf) -> Result<Vec<String>, std::io::Error> {
    let entries = fs::read_dir(dir_path)?; // Read the directory entries.

    let mut subdirs = Vec::new();

    for entry in entries {
        let entry = entry?; // Handle potential I/O errors.
        let path = entry.path();

        if path.is_dir() {
            // Check if the entry is a directory.
            if let Some(subdir_name) = path.file_name() {
                if let Some(subdir_name_str) = subdir_name.to_str() {
                    subdirs.push(subdir_name_str.to_string());
                }
            }
        }
    }

    Ok(subdirs)
}

fn get_repo_path() -> PathBuf {
    match env::var("GOREPO_REPOS_DIR_PATH") {
        Ok(repos_path) => PathBuf::from(repos_path),
        Err(_) => {
            let home_env = env::var("HOME").expect("$HOME not set");
            let mut home_path = PathBuf::from(home_env);
            home_path.push("repos");
            home_path
        }
    }
}

fn main() {
    let cwords = env::var("COMP_WORDS").unwrap();
    let cword = env::var("COMP_CWORD").unwrap();
    let _cline = env::var("COMP_LINE").unwrap();

    let count_word = cword.parse::<usize>().unwrap();
    if count_word > 1 {
        return print!("");
    }

    let repos_path = get_repo_path();
    // Read the directory and handle any potential errors.
    let mut repo_dir_names = read_directory(repos_path).unwrap();
    repo_dir_names.retain(|subdir| !subdir.starts_with('.'));
    repo_dir_names.sort();

    let mut matched_with = Vec::new();

    let last_argument = cwords.split_whitespace().nth(count_word);

    for option in repo_dir_names.iter() {
        if let Some(last) = last_argument {
            if option.contains(last) {
                matched_with.push(option)
            }
        }
    }

    let mut result = String::new();

    if matched_with.len() == 1 {
        result.push_str(matched_with[0]);
    } else if matched_with.len() > 1 {
        result.push('\n');
        for word in matched_with {
            result.push_str(word);
            result.push(' ');
        }
    } else {
        result.push('\n');
        for word in &repo_dir_names {
            result.push_str(word);
            result.push(' ');
        }
    }
    print!("{}", result);
}
