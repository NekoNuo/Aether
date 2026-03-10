use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_shell::ShellExt;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Start the backend sidecar
            let sidecar = app
                .shell()
                .sidecar("aether-server")
                .expect("failed to find aether-server sidecar binary");
            let (_rx, _child) = sidecar.spawn().expect("failed to start aether-server");

            // Create the main window with loading page
            let window =
                WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                    .title("Aether - Starting...")
                    .inner_size(1280.0, 800.0)
                    .center()
                    .on_navigation(|_url| true)
                    .build()?;

            // Wait for backend to be ready, then navigate to it
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
