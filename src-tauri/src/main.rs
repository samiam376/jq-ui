// Prevents additional console window on Windows in release, DO NOT REMOVE!#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::Write,
    process::{Command, Stdio},
};

#[tauri::command]
fn jq(query: &str, json: &str) -> Result<String, String> {
    let mut jq = Command::new("jq")
        .arg(query)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start jq process: {}", e))?;

    {
        let stdin = jq.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin
            .write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write to stdin: {}", e))?;
    }

    let output = jq
        .wait_with_output()
        .map_err(|e| format!("Failed to read jq output: {}", e))?;

    if output.status.success() {
        let stdout =
            String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 output: {}", e))?;

        Ok(stdout)
    } else {
        let stderr =
            String::from_utf8(output.stderr).map_err(|e| format!("Invalid UTF-8 error: {}", e))?;
        Err(stderr)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![jq])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
