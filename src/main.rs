use core::panic;
use std::{
    env,
    process::{Command, Stdio},
};

fn main() {
    let home_env_var: String = env::var("HOME").expect("HOME required");

    let workspace_path: String = format!("{}/workplace", home_env_var);

    let package_subdir_separator = "src";

    let projects_list = find_directories(workspace_path);

    let project_dir = fuzzy_find(projects_list);

    let package_subdir = format!(
        "{}/{}",
        project_dir.replace("\n", ""),
        package_subdir_separator
    );

    let packages_list = find_directories(package_subdir);

    let package_dir = fuzzy_find(packages_list).trim().to_string();

    let package_name = package_dir.split("/").last().unwrap().trim().to_string();

    tmux(&package_dir, &package_name);
}

fn find_directories(base_dir_path: String) -> String {
    let args = vec![
        "-L",
        &base_dir_path,
        "-type",
        "d",
        "-maxdepth",
        "1",
        "-mindepth",
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

fn tmux(session_dir: &String, session_name: &String) {
    //let first_cmd = &format!("cd {} && nvim", session_dir);

    let args = vec![
        "new-session",
        "-s",
        session_name,
        "-c",
        session_dir,
        "nvim",
        ";",
        "split-window",
        "-h",
        ";",
        "select-pane",
        "-L",
    ];

    Command::new("tmux").args(args).status().expect("Failed");
}
