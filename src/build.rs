use std::fs;
use std::path::PathBuf;

fn main() {
    let dest_path = PathBuf::from("src/algorithms/mod.rs");

    let mut mod_file = String::new();
    let algorithms_dir = PathBuf::from("src/algorithms");

    for entry in fs::read_dir(&algorithms_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            if file_name != "mod" {
                // Ajouter chaque fichier comme un module
                mod_file.push_str(&format!("pub mod {};\n", file_name));
            }
        }
    }

    mod_file.push_str("\nuse super::pack::Pack;\n");
    mod_file.push_str("use std::path::Path;\n\n");
    mod_file.push_str("pub fn determine_compressor(mode: &str, files: &[&Path]) -> Result<Box<dyn Pack>, String> {\n");
    mod_file.push_str("    let extension = match mode {\n");
    mod_file.push_str("        \"c\" | \"compress\" | \"p\" | \"pack\" => files[0].extension().and_then(|ext| ext.to_str()),\n");
    mod_file.push_str("        \"u\" | \"uncompress\" | \"unpack\" => files[0].extension().and_then(|ext| ext.to_str()),\n");
    mod_file.push_str("        _ => None,\n");
    mod_file.push_str("    };\n\n");
    mod_file.push_str("    let compressors = vec![\n");

    for entry in fs::read_dir(&algorithms_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            if file_name != "mod" {
                let formatted_name = capitalize_first_letter(file_name);
                mod_file.push_str(&format!(
                    "        Box::new({}::{}) as Box<dyn Pack>,\n",
                    file_name,
                    formatted_name
                ));
            }
        }
    }

    mod_file.push_str("    ];\n\n");
    mod_file.push_str("    for compressor in compressors {\n");
    mod_file.push_str("        if compressor.extensions().contains(&extension.unwrap_or(\"\")) {\n");
    mod_file.push_str("            return Ok(compressor);\n");
    mod_file.push_str("        }\n");
    mod_file.push_str("    }\n\n");
    mod_file.push_str("    let supported_extensions: Vec<&str> = vec![\n");

    for entry in fs::read_dir(&algorithms_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            if file_name != "mod" {
                let formatted_name = capitalize_first_letter(file_name);
                mod_file.push_str(&format!(
                    "        {0}::{1}.extensions(),\n",
                    file_name,
                    formatted_name
                ));
            }
        }
    }

    mod_file.push_str("    ]\n");
    mod_file.push_str("    .into_iter()\n");
    mod_file.push_str("    .flatten()\n");
    mod_file.push_str("    .collect();\n\n");
    mod_file.push_str("    Err(format!(\n");
    mod_file.push_str("        \"Unsupported file format. Use one of the following extensions: {:?}\",\n");
    mod_file.push_str("        supported_extensions\n");
    mod_file.push_str("    ))\n");
    mod_file.push_str("}\n");

    fs::write(dest_path, mod_file).unwrap();
    println!("cargo:rerun-if-changed=src/algorithms");
}

fn capitalize_first_letter(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

