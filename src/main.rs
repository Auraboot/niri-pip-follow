use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn query(what: &str) -> Result<Value, String> {
    let output = Command::new("niri")
        .args(["msg", "--json", what])
        .output()
        .map_err(|e| format!("Не удалось запустить niri: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "Ошибка niri {what}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Ошибка JSON {what}: {e}"))
}

fn follow() -> Result<(), String> {
    let workspaces = query("workspaces")?;
    let windows = query("windows")?;

    let workspace = workspaces
        .as_array()
        .and_then(|items| {
            items.iter().find(|w| w["is_focused"].as_bool() == Some(true))
        });

    let Some(workspace) = workspace else {
        return Ok(());
    };

    // ID нужен для сравнения, а idx — для команды niri.
    let Some(workspace_id) = workspace["id"].as_u64() else {
        return Ok(());
    };
    let Some(workspace_index) = workspace["idx"].as_u64() else {
        return Ok(());
    };

    let Some(windows) = windows.as_array() else {
        return Ok(());
    };

    for window in windows {
        let title = window["title"].as_str().unwrap_or("");
        let app_id = window["app_id"].as_str().unwrap_or("");
        let floating = window["is_floating"].as_bool().unwrap_or(false);

        if title != "Picture in picture" || !app_id.is_empty() || !floating {
            continue;
        }

        if window["workspace_id"].as_u64() == Some(workspace_id) {
            continue;
        }

        let Some(window_id) = window["id"].as_u64() else {
            continue;
        };

        let status = Command::new("niri")
            .args([
                "msg",
                "action",
                "move-window-to-workspace",
                "--window-id",
                &window_id.to_string(),
                "--focus=false",
                &workspace_index.to_string(),
            ])
            .status()
            .map_err(|e| format!("Не удалось переместить окно: {e}"))?;

        if !status.success() {
            return Err(format!("niri не смог переместить окно {window_id}"));
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = Command::new("niri")
        .args(["msg", "--json", "event-stream"])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = stream.stdout.take().ok_or("Нет потока событий niri")?;

    println!("PiP-follow запущен. Остановка: Ctrl+C");

    if let Err(error) = follow() {
        eprintln!("{error}");
    }

    for line in BufReader::new(stdout).lines() {
        let line = line?;
        let Ok(event) = serde_json::from_str::<Value>(&line) else {
            continue;
        };

        if event.get("WorkspaceActivated").is_some()
            || event.get("WorkspacesChanged").is_some()
            || event.get("WindowOpenedOrChanged").is_some()
        {
            if let Err(error) = follow() {
                eprintln!("{error}");
            }
        }
    }

    Ok(())
}
