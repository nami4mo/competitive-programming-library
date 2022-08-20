use std::{env, fs::File, io::Write, path::Path};

// use build.rs for set const vals.
// https://stackoverflow.com/questions/37526598/how-can-i-override-a-constant-via-a-compiler-option

fn main() {
    let consts = [
        ("SAMPLE_A", "usize", "10"),
        ("SAMPLE_B", "u64", "30"),
        ("SAMPLE_C", "f64", "24.5"),
        ("SAMPLE_D", "i64", "-129"),
        ("SAMPLE_PARAM", "i64", "-129"),
        // ("SAMPLE_ERROR", "usize", "12.3"), // error
    ];

    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).expect("Could not create file");

    for &(var_name, var_type, default) in &consts {
        let val = env::var(var_name).unwrap_or(default.to_string());
        let definition = format!("const {}: {} = {};", var_name, var_type, val);
        writeln!(&mut f, "{}", definition).expect("Could not write file");
        println!("cargo:rerun-if-env-changed={}", var_name);
    }
}
