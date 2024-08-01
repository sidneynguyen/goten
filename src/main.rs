use core::panic;
use std::{
    env,
    process::{Command, Stdio},
};

fn main() {
    let home_env_var: String = env::var("HOME").expect("HOME required");

    let workspace_path: String = format!("{}/workplace", home_env_var);

    let projects_list = find_projects(workspace_path);

    let project_dir = fuzzy_find(projects_list);

    println!("{}", project_dir);
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

fn fuzzy_find(fuzzy_find_str: String) -> String {
    let echo_process = Command::new("echo")
        .arg(fuzzy_find_str)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");

    let echo_out = echo_process.stdout.expect("Failed");

    let fzf_process = Command::new("fzf")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");

    let output = fzf_process.wait_with_output().expect("Failed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    return stdout.to_string();
}
