from io import BytesIO
from pathlib import Path
from zipfile import ZipFile

import requests

CURRENT_DIR = Path(__file__).parent.parent.absolute()
VERSION = "0.1.0"

ASSETS_PATH = f"https://github.com/noaione/cmx-ermitteln/releases/download/wasm-{VERSION}/ermitteln-wasm-pkg.zip"

print(f"Downloading assets from {ASSETS_PATH}...")
req = requests.get(ASSETS_PATH)

# Open the file in read mode
file_obj = BytesIO(req.content)
zip_file = ZipFile(file_obj)

# Extract every files into ermitteln-wasm/pkg
print("Extracting files...")
zip_file.extractall(path=CURRENT_DIR / "ermitteln-wasm" / "pkg")
