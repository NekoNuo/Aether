use std::fs;
use std::net::TcpStream;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_shell::ShellExt;

fn get_config_dir(app: &tauri::App) -> PathBuf {
    let dir = app
        .path()
        .app_config_dir()
        .expect("failed to resolve app config dir");
    fs::create_dir_all(&dir).ok();
    dir
}

fn get_env_file_path(app: &tauri::App) -> PathBuf {
    get_config_dir(app).join("aether.env")
}

fn env_file_exists(app: &tauri::App) -> bool {
    get_env_file_path(app).exists()
}

#[tauri::command]
fn save_config(app: tauri::AppHandle, config: String) -> Result<String, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("aether.env");
    fs::write(&path, config).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn load_config(app: tauri::AppHandle) -> Result<String, String> {
    let path = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join("aether.env");
    if path.exists() {
        fs::read_to_string(&path).map_err(|e| e.to_string())
    } else {
        Ok(String::new())
    }
}

#[tauri::command]
fn start_backend(app: tauri::AppHandle) -> Result<(), String> {
    let env_file = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?
        .join("aether.env");

    let mut sidecar = app
        .shell()
        .sidecar("aether-server")
        .map_err(|e| e.to_string())?;

    if env_file.exists() {
        sidecar = sidecar.args(["--env-file", &env_file.to_string_lossy()]);
    }

    let (_rx, _child) = sidecar.spawn().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![save_config, load_config, start_backend])
        .setup(|app| {
            let has_config = env_file_exists(app);

            // If config exists, start backend immediately
            if has_config {
                let env_file = get_env_file_path(app);
                let mut sidecar = app
                    .shell()
                    .sidecar("aether-server")
                    .expect("failed to find aether-server sidecar");
                sidecar = sidecar.args(["--env-file", &env_file.to_string_lossy()]);
                let (_rx, _child) = sidecar.spawn().expect("failed to start aether-server");
            }

            // Decide which page to show
            let start_page = if has_config {
                "index.html" // loading page
            } else {
                "setup.html" // first-time config
            };

            let window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App(start_page.into()),
            )
            .title(if has_config {
                "Aether - Starting..."
            } else {
                "Aether - Setup"
            })
            .inner_size(1280.0, 800.0)
            .center()
            .on_navigation(|_url| true)
            .build()?;

            // If backend was started, wait for it and navigate
            if has_config {
                let window_clone = window.clone();
                thread::spawn(move || {
                    for _ in 0..120 {
                        if TcpStream::connect("127.0.0.1:8084").is_ok() {
                            thread::sleep(Duration::from_millis(500));
                            let _ = window_clone
                                .eval("window.location.replace('http://127.0.0.1:8084')");
                            let _ = window_clone.set_title("Aether");
                            return;
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
