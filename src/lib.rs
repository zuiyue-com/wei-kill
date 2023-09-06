pub fn kill(name: &str) -> Result<String,String> {
    let output = std::process::Command::new("pkill")
        .arg(name)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        Ok(String::from_utf8(output.stdout).unwrap())
    } else {
        Err(String::from_utf8(output.stderr).unwrap())
    }
}
