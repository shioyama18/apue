use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("example/ch11_threads/f05_pthread_cleanup.c")
        .compile("f05_pthread_cleanup");
    cc::Build::new()
        .file("example/ch11_threads/f13_pthread_mutex_timedlock.c")
        .compile("f13_pthread_mutex_timedlock");
    cc::Build::new()
        .file("example/ch11_threads/f16_pthread_barrier.c")
        .compile("f16_pthread_barrier");
    println!("cargo:rerun-if-changed=example/ch11_threads/f05_pthread_cleanup.c");
    println!("cargo:rerun-if-changed=example/ch11_threads/f13_pthread_mutex_timedlock.c");
    println!("cargo:rerun-if-changed=example/ch11_threads/f16_pthread_barrier.c");

    let bindings = bindgen::builder()
        .header_contents("_stdio.h", "#include<stdio.h>\n")
        .header_contents("_stdlib.h", "#include<stdlib.h>\n")
        .header_contents("_setjmp.h", "#include<setjmp.h>\n")
        .header_contents("_signal.h", "#include<signal.h>\n")
        .header_contents("_syslimits.h", "#include<sys/syslimits.h>\n")
        .header_contents("_types.h", "#include<sys/types.h>\n")
        .header_contents("_acct.h", "#include<sys/acct.h>\n")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
