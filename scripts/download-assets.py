import os
import re
import sys
from pathlib import Path
from zipfile import ZipFile

cf_pages = os.getenv("CF_PAGES")
vercel = os.getenv("VERCEL")
gh_runner = os.getenv(key="CI")

VERSION_RE = re.compile(r"^version = \"(.*)\"$")

# Check if we are in the CI
if cf_pages is None and vercel is None and gh_runner is None:
    print("This script should only be run in CI")
    sys.exit(0)


def get_version(cargo_toml: Path) -> str:
    read_lines = cargo_toml.read_text().splitlines()
    for line in read_lines:
        if match := VERSION_RE.match(line):
            return match.group(1)
    raise ValueError("Version not found in Cargo.toml")


CURRENT_DIR = Path(__file__).resolve().parent.parent
cargo_toml = CURRENT_DIR / "Cargo.toml"

VERSION = get_version(cargo_toml)
os.chdir(CURRENT_DIR)

ASSETS_PATH = f"https://github.com/noaione/cmx-ermitteln/releases/download/wasm-{VERSION}/ermitteln-wasm-pkg.zip"

print(f"Downloading assets from {ASSETS_PATH}...")
# Use curl
os.system(f"curl -L {ASSETS_PATH} -o ermitteln-wasm-pkg.zip")

# Open the file in read mode
print("Opening zip file...")
with ZipFile(CURRENT_DIR / "ermitteln-wasm-pkg.zip", "r") as zip_file:
    # Extract every files into ermitteln-wasm/pkg
    print("Extracting files...")
    zip_file.extractall(path=CURRENT_DIR / "ermitteln-wasm" / "pkg")
