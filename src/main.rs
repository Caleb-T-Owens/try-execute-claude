fn main() {
    let first_arg = std::env::args()
        .nth(1)
        .expect("Expected first argument to be CC path");
    std::process::Command::new(&first_arg)
        .arg("--version")
        .spawn()
        .expect("Failed to spawn command")
        .wait()
        .expect("Failed to execute command");
}
