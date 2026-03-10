"""Desktop entry point for Aether backend server.

This script is used by PyInstaller to create a standalone executable.
It starts the FastAPI backend and serves the built frontend as static files.
"""

import os
import sys


def main():
    # When running from PyInstaller bundle, adjust paths
    if getattr(sys, "_MEIPASS", None):
        base_path = sys._MEIPASS
    else:
        base_path = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    sys.path.insert(0, base_path)
    os.chdir(base_path)

    # Import the FastAPI app
    from src.main import app

    # Mount built frontend as static files (API routes take precedence)
    frontend_dist = os.path.join(base_path, "frontend_dist")
    if os.path.isdir(frontend_dist):
        from starlette.staticfiles import StaticFiles

        app.mount(
            "/", StaticFiles(directory=frontend_dist, html=True), name="frontend"
        )

    import uvicorn

    port = int(os.environ.get("PORT", "8084"))
    uvicorn.run(app, host="127.0.0.1", port=port, log_level="info")


if __name__ == "__main__":
    main()
