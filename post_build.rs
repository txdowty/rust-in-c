use std::process::Command;

fn main() {
    println!("post-build");
    build_static_app();
    build_dynamic_app();
}

fn build_static_app() {
        // gcc  main.c -Ltarget/debug -lrust_in_c -o rust-in-c-static
    let output = Command::new("gcc")
        .arg("main.c")
        .arg("-Ibindings")
        .arg("-Ltarget/debug")
        .arg("-l:librust_in_c.a")
        .arg("-o")
        .arg("rust-in-c-static")
        .output()
        .expect("Failed to compile C code");

    if output.status.success() {
        // Convert stdout bytes to a string and print it
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("C compilation output: {}", stdout);
     } else {
        // Convert stderr bytes to a string and print it
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("C compilation error: {}", stderr);
        panic!("C compilation failed");     
     }
}

fn build_dynamic_app() {
        // gcc  main.c -Ltarget/debug -lrust_in_c -o rust-in-c-static
    let output = Command::new("gcc")
        .arg("main.c")
        .arg("-Ibindings")
        .arg("-Ltarget/debug")
        .arg("-lrust_in_c")
        .arg("-o")
        .arg("rust-in-c-dynamic")
        .output()
        .expect("Failed to compile C code");

    if output.status.success() {
        // Convert stdout bytes to a string and print it
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("C compilation output: {}", stdout);
     } else {
        // Convert stderr bytes to a string and print it
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("C compilation error: {}", stderr);
        panic!("C compilation failed");     
     }
}