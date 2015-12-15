use std::process::Command;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

const RUST_DIR: &'static str = "./rust";

fn ensure_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    match fs::read_dir(path) {
        Ok(..) => Ok(()),
        Err(..) => fs::create_dir(path),
    }
}

fn main() {
    // Run rustc to get the version
    let rustc_output = Command::new("rustc").arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute rustc: {}", e);
        });
    let output_bytes: &[u8] = rustc_output.stdout.as_ref();
    let version = match std::str::from_utf8(output_bytes) {
        Ok(s) => s.split(" ").nth(2).expect("rustc gave invalid version format"),
        Err(e) => panic!(e),
    }.trim_left_matches("(");

    match fs::metadata(Path::new(RUST_DIR).join(version)) {
        Ok(meta) => {
            if !meta.is_file() {
                match fs::remove_dir_all(RUST_DIR) {
                    Err(e) => panic!(e),
                    _ => {},
                }
            }
        },
        Err(..) => {}, // Ok to ignore since this just means that the version file doesn't exist
    }

    // Ensure the rust directory exists
    if let Err(e) = ensure_dir(RUST_DIR) {
        panic!(e);
    }

    // cd to the rust directory
    if let Err(e) = env::set_current_dir(RUST_DIR) {
        panic!(e);
    }

    // Shell out to perform the build.  In the future, the logic
    // to grab libcore could be done in rust in order to support
    // platforms without a posix shell
    Command::new("sh")
        .arg("../build.sh")
        .env("DOWNLOAD_LINK",
             format!("https://github.com/rust-lang/rust/tarball/{}", version))
        .status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e);
        });

    // Now that we've successfully unzipped, write the version
    // file as a success tag.
    match fs::File::create(version) {
        Err(e) => panic!(e),
        _ => {},
    }
}
