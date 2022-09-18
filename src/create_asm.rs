pub struct CreateAsm(Option<String>, Option<String>);

impl CreateAsm {
    pub fn new(file1: Option<String>, file2: Option<String>) -> Self {
        Self(file1, file2)
    }

    pub fn is_ok(&self) -> bool {
        self.0.is_some() && self.1.is_some()
    }

    pub fn get_first(&self) -> Option<&str> {
        self.0.as_deref()
    }

    pub fn get_second(&self) -> Option<&str> {
        self.1.as_deref()
    }

    pub fn cleanup(&self) {
        if let Some(file) = &self.0 {
            std::fs::remove_file(file).expect(&format!("could not remove file {}", file));
        }
        if let Some(file) = &self.1 {
            std::fs::remove_file(file).expect(&format!("could not remove file {}", file));
        }
    }
}

pub fn init() -> CreateAsm {
    let file1 = get_nth_arg(1);
    let file2 = get_nth_arg(2);

    println!("file1: {}, file2: {}", file1, file2);

    let asm1 = create_asm_for_arg(&file1);
    let asm2 = create_asm_for_arg(&file2);

    CreateAsm::new(asm1, asm2)
}

fn get_timestamp() -> u128 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos()
}

fn get_file_name_from_arg(arg_file_name: &str) -> &str {
    std::path::Path::new(arg_file_name)
        .file_name()
        .expect("could not get filename")
        .to_str()
        .expect("could not convert OsStr to string")
}

fn get_nth_arg(num: usize) -> String {
    std::env::args()
        .skip(num)
        .next()
        .expect(&format!("missing file name #{}", num))
}

fn create_asm_for_arg(arg_filename: &str) -> Option<String> {
    let out = std::process::Command::new("objdump")
        .arg("--no-show-raw-insn")
        .arg("-w")
        .arg("-d")
        .arg("-Mintel")
        .arg(&arg_filename)
        .output()
        .expect(&format!("failed to run objdump on {}", arg_filename));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    let time = get_timestamp();
    if !stderr.is_empty() {
        println!("err:\n\n{}", stderr);
        None
    } else {
        use std::borrow::Borrow;
        use std::io::prelude::*;
        let filename = format!(
            "/var/tmp/{}-{}.asm",
            get_file_name_from_arg(&arg_filename),
            time
        );
        let mut file = std::fs::File::create(&filename)
            .expect(&format!("failed to create file {}", &filename));

        let content: &str = stdout.borrow();
        file.write_all(content.as_bytes())
            .expect(&format!("failed to write asm to file {}", filename));

        println!("output saved to {}", filename);
        Some(filename)
    }
}
