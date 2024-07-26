import os
import sys
from pathlib import Path
from zipfile import ZipFile

cf_pages = os.getenv("CF_PAGES")
vercel = os.getenv("VERCEL")
gh_runner = os.getenv(key="CI")

# Check if we are in the CI
if cf_pages is None and vercel is None and gh_runner is None:
    print("This script should only be run in CI")
    sys.exit(0)

CURRENT_DIR = Path(__file__).parent.parent.absolute()
VERSION = "0.1.3"
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
