pub fn run() {
    tauri::Builder::default()
        .on_navigation(|_webview, _url| {
            // Allow navigation to external URLs (Aether backend)
            true
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
