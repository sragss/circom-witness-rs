use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    if cfg!(feature = "build-circom") {
        let circuit_path = env::var("CIRC_RS_CIRCUIT_PATH").expect("CIRC_RS_CIRCUIT_PATH should be set for circom-witness-rs build-circom feature. Set env variable in build.rs");
        let circuit_file = Path::new(&circuit_path);

        let status = Command::new("circom")
            .args([
                fs::canonicalize(circuit_file).unwrap().to_str().unwrap(),
                "--c",
            ])
            .status()
            .unwrap();
        assert!(status.success());
    }
    if cfg!(feature = "build-witness") {
        // let cpp_path = env::var("CIRC_RS_CPP_PATH").expect("CIRC_RS_CPP_PATH should be set for circom-witness-rs build-witness feature. Set env variable in build.rs");

        // let cpp = PathBuf::from(cpp_path);

        // println!("cargo:warning=\"{}\"", cpp.to_str().unwrap());

        // let status = Command::new("./script/replace.sh")
        //     .arg(cpp.to_str().unwrap())
        //     .status()
        //     .unwrap();
        // assert!(status.success());

        // cxx_build::bridge("src/generate.rs")
        //     .file("src/circuit.cc")
        //     .flag_if_supported("-std=c++14")
        //     .flag_if_supported("-w")
        //     .flag_if_supported("-d")
        //     .flag_if_supported("-g")
        //     .compile("witness");

        // println!("cargo:rerun-if-changed=src/main.rs");
        // println!("cargo:rerun-if-changed=src/circuit.cc");
        // println!("cargo:rerun-if-changed=include/circuit.h");
    }

    // let replace_script_path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/script/replace.sh";
    // println!("cargo:rustc-env=CIRC_RS_REPLACE_SCRIPT_PATH={}", replace_script_path);
    // println!("cargo:warning=\"setting to {}\"", replace_script_path);
}
