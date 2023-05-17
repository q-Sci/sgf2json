use glob::glob;
use std::fs;

fn main() {
    // create list of all files inside the folder
    let mut sgf_files: Vec<String> = Vec::new();
    for file in glob("./src/sgf/*.sgf").expect("Failed to read glob pattern") {
        sgf_files.push(file.unwrap().display().to_string());
    }

    // print the content of each sgf file
    for file in sgf_files {
        let file_contents =
            fs::read_to_string(file).expect("Failed to read file");
        println!("-----\n{file_contents}\n-----\n");
    }
}
