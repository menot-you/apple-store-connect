"""asc-mcp — MCP server for Apple App Store Connect API."""

import os
import platform
import subprocess
import sys
from pathlib import Path

__version__ = "0.1.0"

BINARY = "asc-mcp"
REPO = "menot-you/asc-mcp"


def _get_platform() -> str:
    system = platform.system().lower()
    machine = platform.machine().lower()

    if system == "darwin" and machine == "arm64":
        return "macos-arm64"
    if system == "linux" and machine in ("x86_64", "amd64"):
        return "linux-x86_64"

    raise RuntimeError(
        f"Unsupported platform: {system}-{machine}. "
        f"Supported: darwin-arm64, linux-x86_64. "
        f"Install from source: cargo install asc-mcp"
    )


def _binary_path() -> Path:
    """Return the path to the bundled binary, downloading if necessary."""
    pkg_dir = Path(__file__).parent
    dest = pkg_dir / BINARY

    if dest.exists():
        return dest

    # Download from GitHub Releases
    import urllib.request

    plat = _get_platform()
    artifact = f"{BINARY}-{plat}"
    url = f"https://github.com/{REPO}/releases/download/v{__version__}/{artifact}"

    print(f"asc-mcp: downloading {BINARY} v{__version__} for {plat}...", file=sys.stderr)

    try:
        with urllib.request.urlopen(url) as response:  # noqa: S310
            data = response.read()
        dest.write_bytes(data)
        dest.chmod(0o755)
        size_mb = len(data) / 1024 / 1024
        print(f"asc-mcp: installed {BINARY} ({size_mb:.1f} MB)", file=sys.stderr)
    except Exception as exc:
        raise RuntimeError(
            f"asc-mcp: failed to download binary — {exc}\n"
            f"Install from source: cargo install asc-mcp"
        ) from exc

    return dest


def main() -> None:
    """Entry point — exec the binary with forwarded args."""
    binary = _binary_path()
    os.execv(str(binary), [str(binary), *sys.argv[1:]])
