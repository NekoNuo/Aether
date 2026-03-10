use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Allow navigation to external URLs on the main window
            let main_window = app.get_webview_window("main").unwrap();
            main_window.on_navigation(|url| {
                let _ = url; // allow all URLs
                true
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
