use std::{env, fs};

fn main() {
    let paths = fs::read_dir("../proto-files").unwrap();

    let proto_files_dir = format!("{}/proto-files", env::var("OUT_DIR").unwrap());
    fs::create_dir(env::var("OUT_DIR").unwrap() + "/proto-files").unwrap();

    for path in paths {
        let path = path.unwrap();
        tonic_build::compile_protos(format!("{}", path.path().display())).unwrap();
        fs::copy(
            format!("{}", path.path().display()),
            &format!("{}/{}", proto_files_dir, path.file_name().to_str().unwrap()),
        )
        .unwrap();
    }
}
