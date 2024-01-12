#[allow(non_snake_case)] // For clean imports
pub mod DefaultPaths {
    use std::path::{PathBuf, Path};

    /// Defaults to <build target dir>/circom
    pub fn artifact_root() -> PathBuf {
        let cargo_target_dir = std::env::var("OUT_DIR").expect("Failed to get OUT_DIR");
        Path::new(&cargo_target_dir).join("circom").to_path_buf()
    }

    pub fn cpp_artifacts_root() -> PathBuf {
        artifact_root().join("/cpp")
    }

    /// Defaults to <build target dir>/circom/cpp/circuit.cc
    pub fn updated_cpp() -> PathBuf {
        cpp_artifacts_root().join("circuit.cc")
    }

    pub fn updated_consts() -> PathBuf {
        cpp_artifacts_root().join("constants.dat")
    }

    pub fn temp_cpp() -> PathBuf {
        artifact_root().join("temp.cc").to_path_buf()
    }

    pub fn replace_script() -> PathBuf {
        // let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR");
        PathBuf::from("./").join("/script/replace.sh")
    }

}