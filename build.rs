use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::builder()
        .header_contents("_stdio.h", "#include<stdio.h>\n")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
