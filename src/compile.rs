use crate::default_paths::DefaultPaths;
use std::{env, fs, path::PathBuf, process::Command};

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

    let status = Command::new("circom")
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
pub fn compile_cpp(cpp_path: PathBuf, dat_path: PathBuf) {
    run_replace(cpp_path, dat_path);
    cxx_build();
}

fn replace_script(cpp_path: PathBuf) {
    if !cpp_path.exists() {
        panic!("cpp file not found: {:?}", cpp_path);
    }
    println!("cargo:warning=\"{}\"", cpp_path.to_str().unwrap());

    let output_path = DefaultPaths::updated_cpp();
    if !output_path.parent().unwrap().exists() {
        fs::create_dir_all(output_path.parent().unwrap()).unwrap();
    }

    let replace_script = include_str!("../script/replace.sh");
    let mut command = Command::new("sh");
    command.arg("-c").arg(replace_script);
    command.arg(cpp_path.to_str().unwrap());
    command.arg(output_path.to_str().unwrap());

    let output = match command.output() {
        Ok(output) => output,
        Err(e) => {
            panic!("Failed to execute command: {}", e);
        }
    };
    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }
}

fn cxx_build() {
    // Target env var will be present if run from build.rs
    if env::var("TARGET").is_err() {
        panic!("TARGET env var is not set – are you attempting to build at runtime?");
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

fn run_replace(cpp_path: PathBuf, dat_path: PathBuf) {
    use std::fs::File;
    use std::io::Write;

    if !cpp_path.exists() {
        panic!("cpp file not found: {:?}", cpp_path);
    }

    let filename_without_extension = cpp_path.file_stem().unwrap().to_str().unwrap();

    // TODO(sragss): Move to artifacts file.
    let mut temp_file = File::create(DefaultPaths::temp_cpp()).unwrap();

    let header = r#"#include "witness/include/witness.h"
#include "witness/src/generate.rs.h"

/// We need this accessor since cxx doesn't support hashmaps yet
class IOSignalInfoAccessor {
private:
  Circom_CalcWit *calcWitContext;

public:
  explicit IOSignalInfoAccessor(Circom_CalcWit *calcWit)
      : calcWitContext(calcWit) {}
  auto operator[](size_t index) const -> decltype(auto) {
    return (calcWitContext
                ->templateInsId2IOSignalInfoList)[index % get_size_of_input_hashmap()];
  }
};

typedef void (*Circom_TemplateFunction)(uint __cIdx, Circom_CalcWit* __ctx);

//////////////////////////////////////////////////////////////////
/// Generated code from circom compiler below
//////////////////////////////////////////////////////////////////
"#;

    temp_file.write_all(header.as_bytes()).unwrap();

    let output = Command::new("sed")
        .arg("-e")
        .arg("s/FrElement\\* signalValues/rust::Vec<FrElement> \\&signalValues/g")
        .arg("-e")
        .arg("s/std::string/rust::string/g")
        .arg("-e")
        .arg("s/ctx->templateInsId2IOSignalInfo/IOSignalInfoAccessor(ctx)/g")
        .arg("-e")
        .arg("s/u32\\* mySubcomponents/rust::Vec<u32> mySubcomponents/g")
        .arg("-e")
        .arg("s/FrElement\\* circuitConstants/rust::Vec<FrElement> \\&circuitConstants/g")
        .arg("-e")
        .arg("s/rust::string\\* listOfTemplateMessages/rust::Vec<rust::string> \\&listOfTemplateMessages/g")
        .arg("-e")
        .arg("s/FrElement expaux\\[\\([0-9]*\\)\\];/rust::Vec<FrElement> expaux = create_vec(\\1);/g")
        .arg("-e")
        .arg("s/FrElement lvar\\[\\([0-9]*\\)\\];/rust::Vec<FrElement> lvar = create_vec(\\1);/g")
        .arg("-e")
        .arg("s/FrElement lvarcall\\[\\([0-9]*\\)\\];/rust::Vec<FrElement> lvarcall = create_vec(\\1);/g")
        .arg("-e")
        .arg("s/PFrElement aux_dest/FrElement \\*aux_dest/g")
        .arg("-e")
        .arg("s/subcomponents = new uint\\[\\([0-9]*\\)\\];/subcomponents = create_vec_u32(\\1);/g")
        .arg("-e")
        .arg("/trace/d")
        .arg("-e")
        .arg("s/\\(ctx,\\)\\(lvarcall,\\)\\(myId,\\)/\\1\\&\\2\\3/g")
        .arg("-e")
        .arg("/^#include/d")
        .arg("-e")
        .arg("/assert/d")
        .arg("-e")
        .arg("/mySubcomponentsParallel/d")
        .arg("-e")
        .arg("s/FrElement lvarcall\\[\\([0-9]*\\)\\];/rust::Vec<FrElement> lvarcall = create_vec(\\1);/g")
        .arg("-e")
        .arg("s/,FrElement\\* lvar,/,rust::Vec<FrElement>\\& lvar,/g")
        .arg("-e")
        .arg("s/ctx,\\&lvarcall,myId,/ctx,lvarcall,myId,/g")
        .arg("-e")
        .arg("/delete/{N;d;}")
        .arg("-e")
        .arg("N;/\\ndelete/!P;D")
        .arg("-e")
        .arg("/^#include/d")
        .arg(cpp_path.to_str().unwrap())
        .output()
        .expect("failed to execute process");

    temp_file.write_all(&output.stdout).unwrap();

    let output = Command::new("sed")
        .arg("-E")
        .arg("-e")
        .arg("s/\"([^\\\"]+)\"\\+ctx->generate_position_array\\(([^)]+)\\)/generate_position_array(\"\\1\", \\2)/g")
        .arg("-e")
        .arg("s/subcomponents = new uint\\[([0-9]+)\\]\\{0\\};/subcomponents = create_vec_u32(\\1);/g")
        .arg("-e")
        .arg("s/^uint aux_dimensions\\[([0-9]+)\\] = \\{([^}]+)\\};$/rust::Vec<uint> aux_dimensions = rust::Vec<uint32_t>{\\2};/")
        .arg(format!("{}.new", filename_without_extension))
        .output()
        .expect("failed to execute process");

    let mut file = File::create(DefaultPaths::updated_cpp()).unwrap();
    file.write_all(&output.stdout).unwrap();

    Command::new("cp")
        .arg(dat_path)
        .arg("src/constants.dat")
        .status()
        .expect("failed to copy file");
}
