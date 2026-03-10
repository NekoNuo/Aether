use tauri::{WebviewUrl, WebviewWindowBuilder};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Create window programmatically so we can set on_navigation
            // to allow navigating to external Aether backend URLs
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Aether")
                .inner_size(1280.0, 800.0)
                .center()
                .on_navigation(|_url| true)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
