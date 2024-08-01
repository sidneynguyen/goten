use core::panic;
use std::{env, process::Command};

fn main() {
    let home_env_var: String = env::var("HOME").expect("HOME required");

    let workspace_path: String = format!("{}/workplace", home_env_var);

    let projects_list = find_projects(workspace_path);

    println!("{}", projects_list);
}

fn find_projects(workspace_path: String) -> String {
    let args = vec![
        "-L",
        &workspace_path,
        "-type",
        "d",
        "-maxdepth",
        "1",
        "-not",
        "-path",
        "*/.*",
    ];

    let output = Command::new("find").args(&args).output().expect("Failed");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.to_string();
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("{}", stderr);
    }
}
