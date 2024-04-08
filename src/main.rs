use dialoguer::{Input, Select};
use std::path::Path;
use std::process::Command;

fn main() {
    // Get the input file path from the user
    let file_path: String = Input::new()
        .with_prompt("Enter the file path:")
        .interact_text()
        .expect("Failed to read input");

    // Check if the file exists
    if !Path::new(&file_path).exists() {
        println!("File does not exist");
        return;
    }

    // Ask the user for the desired output format
    let formats = &["pdf", "docx"]; // Add more formats as needed
    let format_index = Select::new()
        .items(formats)
        .default(0)
        .with_prompt("Select output format:")
        .interact()
        .unwrap_or(0);

    let output_format = formats[format_index];

    // Get the output file path
    let output_path = format!("output.{}", output_format);

    // Convert the file using Pandoc
    let output = Command::new("pandoc")
        .args(&["-s", &file_path, "-o", &output_path])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!(
            "Conversion successful. Output file saved as: {}",
            output_path
        );
    } else {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
