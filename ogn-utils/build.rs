use std::{
    io::Read,
    process::{Command, Stdio},
};

fn main() {
    println!("cargo:rerun-if-changed=src/hello.c");
    let mut pdftotext_path = String::new();
    Command::new("nim")
        .arg("r")
        .arg("download_pdftotext.nim")
        .stdout(Stdio::piped())
        .spawn()
        .expect("couldn't run nim compiler")
        .stdout
        .expect("couldn't get stdout from pdftotext downloader")
        .read_to_string(&mut pdftotext_path)
        .unwrap();

    println!("cargo:rustc-env={}={}", "PDF2TEXT_PATH", pdftotext_path);
}
