use std::fs;

fn main() {
    let paths = fs::read_dir("../proto-files").unwrap();


    for path in paths {
        tonic_build::compile_protos(format!("{}", path.unwrap().path().display())).unwrap();
    }
}
