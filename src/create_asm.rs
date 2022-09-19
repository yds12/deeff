pub fn create_asm_for_arg(arg_filename: &str) -> Option<std::process::Output> {
    let out = std::process::Command::new("objdump")
        .arg("--no-show-raw-insn")
        .arg("-w")
        .arg("-d")
        .arg("-Mintel")
        .arg(&arg_filename)
        .output()
        .expect(&format!("failed to run objdump on {}", arg_filename));
    let stderr = String::from_utf8_lossy(&out.stderr);

    if !stderr.is_empty() {
        println!("err:\n\n{}", stderr);
        None
    } else {
        Some(out)
    }
}
