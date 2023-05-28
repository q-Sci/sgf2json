use glob::glob;
use std::fs;

fn main() {
    // create list of all files inside the folder
    let mut sgf_files: Vec<String> = Vec::new();
    for file in glob("./src/sgf/*.sgf").expect("Failed to read glob pattern") {
        sgf_files.push(file.unwrap().display().to_string());
    }

    let mut output_data: Vec<String> = Vec::new(); // content written to json file, not formatted yet

    // print the content of each sgf file
    for file in sgf_files {
        let file_contents = fs::read_to_string(file).expect("Failed to read file");
        output_data.push(file_contents.to_string());
    }

    let mut formatted_output_data: Vec<String> = Vec::new(); // formatted content for the json file (correct syntax)

    // formatting the content to json format
    formatted_output_data.push(concat!("{\n", r#"  "problems": ["#).to_owned()); // first line of json
    for problem in &output_data {
        let index = &output_data.iter().position(|r| r == problem).unwrap() + 1; // id of the problem

        let mut obj = concat!("    {\n", r#"      "id": "#).to_string();
        obj.push_str(&index.to_string());
        obj.push_str(",\n");
        obj.push_str(r#"      "problem": "#);
        obj.push_str(&problem);
        obj.push_str("\n    },");
        formatted_output_data.push(obj.to_owned());
    }
    formatted_output_data.push("  ],\n}".to_owned()); // last line of json

    fs::write(
        "./src/output/problems.json",
        formatted_output_data.join("\n"),
    )
    .expect("Unable to write file");
}
