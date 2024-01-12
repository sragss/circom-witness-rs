use std::{fs, path::PathBuf, process::Command, env};
use crate::default_paths::DefaultPaths;

/// Runs circom compile, panics if failure.
pub fn compile_circom(circuit_path: PathBuf) {
    let circuit_file = circuit_path;

    let circuit_file_path = fs::canonicalize(&circuit_file).unwrap();
    if !circuit_file_path.exists() {
        panic!("Circuit file not found: {:?}", circuit_file_path);
    }

    let artifact_root_path = DefaultPaths::artifact_root();
    if !artifact_root_path.exists() {
        fs::create_dir_all(&artifact_root_path).unwrap();
    }

    let status =
        Command::new("circom")
            .args([
                circuit_file_path.to_str().unwrap(),
                "--c",
                "-o",
                artifact_root_path.to_str().unwrap(),
            ])
            .status()
            .unwrap();
    assert!(status.success());
}

/// Compiles the cpp binary.
pub fn compile_cpp(circuit_path: PathBuf) {
    replace_script(circuit_path);
    cxx_build();
}

fn replace_script(circuit_path: PathBuf) {
    let circuit_name = circuit_path.file_stem().unwrap().to_str().unwrap().to_string();
    let cpp_path = DefaultPaths::initial_cpp(&circuit_name);
    if !cpp_path.exists() {
        panic!("cpp file not found: {:?}", cpp_path);
    }
    println!("cargo:warning=\"{}\"", cpp_path.to_str().unwrap());

    let mut command = Command::new(DefaultPaths::replace_script());
    command.arg(cpp_path.to_str().unwrap());

    let output_path = DefaultPaths::updated_cpp();
    if !output_path.parent().unwrap().exists() {
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    }
    command.arg(output_path.to_str().unwrap());

    let output = command.output().unwrap();
    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Failed to execute command");
    }
}

fn cxx_build() {
    // Set TARGET environment variable if not present
    if env::var("TARGET").is_err() {
        env::set_var("TARGET", "some target thing");
    }

    let cpp_file = DefaultPaths::updated_cpp();
    cxx_build::bridge("src/generate.rs")
        .file(cpp_file.to_str().unwrap())
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-w")
        .flag_if_supported("-d")
        .flag_if_supported("-g")
        .compile("witness");
}