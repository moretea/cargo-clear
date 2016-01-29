use std::process::Command;
fn main() {
    let mut child = Command::new("tput").arg("reset").spawn().unwrap_or_else(|e| panic!("Could not run tput: {}", e));
    let result = child.wait().unwrap_or_else(|e| panic!("Could not wait for tput process: {}", e));

    if ! result.success() {
      panic!("tput failed with error code {}", result.code().unwrap());
    }
}
