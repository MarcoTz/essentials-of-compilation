use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub const EXAMPLES_DIR: &str = "examples";

pub struct Example {
    pub name: String,
    pub source: String,
    pub path: PathBuf,
    pub expected: String,
}

pub fn load_examples() -> Result<Vec<Example>, Box<dyn std::error::Error>> {
    println!("Loading examples from {EXAMPLES_DIR}\n");
    let dir_contents = read_dir(EXAMPLES_DIR)?;
    let mut examples = vec![];
    for entry in dir_contents {
        let entry = entry?;
        let example_dir = entry.path();
        let example_base = example_dir.file_stem().unwrap();
        let mut example_source = example_dir.join(example_base);
        example_source.set_extension("lang");
        let example_contents = read_to_string(&example_source)?;

        let mut example_expected = example_dir.join(example_base);
        example_expected.set_extension("expected");
        let expected = read_to_string(example_expected)?;

        examples.push(Example {
            name: example_base.to_str().unwrap().to_owned(),
            source: example_contents,
            path: example_source,
            expected,
        });
    }
    Ok(examples)
}
