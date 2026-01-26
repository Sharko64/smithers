#!/usr/bin/env sh
set -eu

PROJECT_NAME="smithers"
INSTALL_DIR="$HOME/.local/bin"
BIN_PATH="$INSTALL_DIR/$PROJECT_NAME"
PYTHON_MIN_VERSION="3.10"

echo "▶ Installing $PROJECT_NAME..."

# ----------------------------
# Check dependencies
# ----------------------------
if ! command -v python3 >/dev/null 2>&1; then
  echo "✗ python3 is required but not installed."
  exit 1
fi

PYTHON_VERSION=$(python3 - <<EOF
import sys
print(f"{sys.version_info.major}.{sys.version_info.minor}")
EOF
)

echo "✓ Found python $PYTHON_VERSION"

# ----------------------------
# Create install directory
# ----------------------------
mkdir -p "$INSTALL_DIR"

# ----------------------------
# Install using uv if available
# ----------------------------
if command -v uv >/dev/null 2>&1; then
  echo "✓ Using uv"
  uv tool install smithers --force
else
  echo "⚠ uv not found, falling back to pip"
  python3 -m pip install --user --upgrade smithers
fi

# ----------------------------
# Ensure binary exists
# ----------------------------
if [ ! -x "$BIN_PATH" ]; then
  echo "✗ Installation failed: $BIN_PATH not found"
  exit 1
fi

echo "✓ Installed to $BIN_PATH"

# ----------------------------
# PATH check
# ----------------------------
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo ""
  echo "⚠ WARNING: $INSTALL_DIR is not in your PATH"
  echo "Add this to your shell config:"
  echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

# ----------------------------
# Optional system-wide symlink
# ----------------------------
if [ "$(id -u)" -ne 0 ]; then
  echo ""
  echo "▶ Optional: create /usr/bin/$PROJECT_NAME symlink"
  echo "This requires sudo."
  printf "Create symlink? [y/N]: "
  read answer

  if [ "$answer" = "y" ] || [ "$answer" = "Y" ]; then
    sudo ln -sf "$BIN_PATH" "/usr/bin/$PROJECT_NAME"
    echo "✓ Symlink created: /usr/bin/$PROJECT_NAME → $BIN_PATH"
  fi
fi

echo ""
echo "🎉 $PROJECT_NAME installed successfully!"
echo "Run: $PROJECT_NAME --help"