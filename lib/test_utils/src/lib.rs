use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub const EXAMPLES_DIR: &str = "examples";

pub struct Example {
    pub name: String,
    pub source: String,
    pub path: PathBuf,
}

pub fn load_examples() -> Result<Vec<Example>, Box<dyn std::error::Error>> {
    println!("Loading examples from {EXAMPLES_DIR}\n");
    let dir_contents = read_dir(EXAMPLES_DIR)?;
    let mut examples = vec![];
    for entry in dir_contents {
        let entry = entry?;
        let example_contents = read_to_string(entry.path())?;
        let example_name = entry
            .path()
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not get file name as string")
            .to_owned();
        examples.push(Example {
            name: example_name,
            source: example_contents,
            path: entry.path(),
        });
    }
    Ok(examples)
}
