"""Desktop entry point for Aether backend server.

This script is used by PyInstaller to create a standalone executable.
It starts the FastAPI backend and serves the built frontend as static files.
Configuration is loaded from a .env file passed via --env-file argument.
"""

import os
import sys


def load_env_file(path):
    """Load environment variables from a .env file."""
    if not os.path.isfile(path):
        return
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if "=" in line:
                key, _, value = line.partition("=")
                key = key.strip()
                value = value.strip().strip("'\"")
                os.environ.setdefault(key, value)


def main():
    # When running from PyInstaller bundle, adjust paths
    if getattr(sys, "_MEIPASS", None):
        base_path = sys._MEIPASS
    else:
        base_path = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    sys.path.insert(0, base_path)
    os.chdir(base_path)

    # Load .env file from --env-file argument or default location
    env_file = None
    for i, arg in enumerate(sys.argv):
        if arg == "--env-file" and i + 1 < len(sys.argv):
            env_file = sys.argv[i + 1]
            break
    if env_file:
        load_env_file(env_file)

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
