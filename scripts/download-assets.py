import os
from pathlib import Path
from zipfile import ZipFile

CURRENT_DIR = Path(__file__).parent.parent.absolute()
VERSION = "0.1.1"
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
