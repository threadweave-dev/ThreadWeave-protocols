use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from("proto");
    let mut protos = Vec::new();
    collect_protos(&proto_root, &mut protos)?;

    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: Cargo executes build scripts in a single-threaded process.
    unsafe { env::set_var("PROTOC", protoc) };

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .include_file("threadweave_protocols.rs")
        .compile_protos(&protos, &[proto_root])?;
    println!("cargo:rerun-if-changed=proto");
    Ok(())
}

fn collect_protos(dir: &PathBuf, protos: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_protos(&path, protos)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "proto")
        {
            protos.push(path);
        }
    }
    protos.sort();
    Ok(())
}
