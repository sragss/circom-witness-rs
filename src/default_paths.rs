#[allow(non_snake_case)] // For clean imports
pub mod DefaultPaths {
    use std::path::{PathBuf, Path};

    /// Defaults to <build target dir>/circom
    pub fn artifact_root() -> PathBuf {
        let cargo_target_dir = std::env::var("OUT_DIR").expect("Failed to get OUT_DIR");
        Path::new(&cargo_target_dir).join("circom").to_path_buf()
    }

    /// Defaults to <build target dir>/circom/<circuit_name>_cpp/<circuit_name>.cpp
    pub fn initial_cpp(circuit_name: &str) -> PathBuf {
        let cpp = artifact_root()
            .join(circuit_name.to_owned() + "_cpp")
            .join(circuit_name.to_owned() + ".cpp");
        cpp
    }

    /// Defaults to <build target dir>/circom/cpp/circuit.cc
    pub fn updated_cpp() -> PathBuf {
        artifact_root().join("cpp/circuit.cc").to_path_buf()
    }

    pub fn replace_script() -> PathBuf {
        Path::new("./script/replace.sh").to_path_buf()
    }

}