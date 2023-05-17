use glob::glob;
use std::fs;

fn main() {
    // create list of all files inside the folder
    let mut sgf_files: Vec<String> = Vec::new();
    for file in glob("./src/sgf/*.sgf").expect("Failed to read glob pattern") {
        sgf_files.push(file.unwrap().display().to_string());
    }

    let mut output_data: Vec<String> = Vec::new(); // content written to json file

    // print the content of each sgf file
    for file in sgf_files {
        let file_contents =
            fs::read_to_string(file).expect("Failed to read file");
        output_data.push(file_contents.to_string());
    }

    fs::write("./src/output/problems.json", output_data.join("\n")).expect("Unable to write file");
}
