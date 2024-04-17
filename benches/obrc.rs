use std::process::Command;

fn main() {
    let hash = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap();
    let hash = String::from_utf8(hash.stdout).unwrap();

    let tree_dirty = Command::new("git").arg("diff").output().unwrap();
    let tree_dirty = if tree_dirty.stdout.is_empty() {
        ""
    } else {
        "(tree dirty)"
    };

    println!("Hello world!: {} {tree_dirty}", hash.trim());
}

