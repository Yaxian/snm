


set -e

RELEASE="latest"
OS="$(uname -s)"

if [ -d "$HOME/.snm" ]; then
  INSTALL_DIR="$HOME/.snm"
elif [ -n "$XDG_DATA_HOME" ]; then
  INSTALL_DIR="$XDG_DATA_HOME/snm"
elif [ "$OS" = "Darwin" ]; then
  INSTALL_DIR="$HOME/Library/Application Support/snm"
else
  INSTALL_DIR="$HOME/.local/share/snm"
fi



set_filename() {
  if [ "$OS" = "Linux" ]; then
    # Based on https://stackoverflow.com/a/45125525
    case "$(uname -m)" in
      arm | armv7*)
        echo "OS $OS is not supported."
        exit 1
        ;;
      aarch* | armv8*)
        FILENAME="snm-linux-x86_64.zip"
        ;;
      *)
        FILENAME="snm-linux-x86_64.zip"
    esac
  elif [ "$OS" = "Darwin" ] && [ "$FORCE_INSTALL" = "true" ]; then
    FILENAME="fnm-macos"
    USE_HOMEBREW="false"
    if [ "$(uname -m)" = "arm64" ]; then
        FILENAME="snm-macos-arm.zip"
    else
        FILENAME="snm-macos-x86_64.zip"
    fi
    echo "Downloading the latest fnm binary from GitHub..."
    # echo "  Pro tip: it's easier to use Homebrew for managing fnm in macOS."
    # echo "           Remove the \`--force-no-brew\` so it will be easy to upgrade."
  elif [ "$OS" = "Darwin" ]; then
    # USE_HOMEBREW="true"
    # echo "Downloading fnm using Homebrew..."
    echo "not supported homebrew"
    exit 1
  else
    echo "OS $OS is not supported."
    echo "If you think that's a bug - please file an issue to https://github.com/Schniz/fnm/issues"
    exit 1
  fi
}



download_snm(){
    URL="https://github.com/sheinsight/snm/releases/latest/download/$FILENAME"
    DOWNLOAD_DIR=$(mktemp -d)
    echo "Downloading $URL..."
    mkdir -p "$INSTALL_DIR" &>/dev/null
        if ! curl --progress-bar --fail -L "$URL" -o "$DOWNLOAD_DIR/$FILENAME"; then
      echo "Download failed.  Check that the release/filename are correct."
      exit 1
    fi
    unzip -q "$DOWNLOAD_DIR/$FILENAME" -d "$DOWNLOAD_DIR"

    for file in "$DOWNLOAD_DIR"/archive/*; do
        chmod u+x "$file"
        mv "$file" "$INSTALL_DIR"
    done

    echo "Downloaded to $DOWNLOAD_DIR"
}


check_dependencies() {
  echo "Checking dependencies for the installation script..."

  echo -n "Checking availability of curl... "
  if hash curl 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  echo -n "Checking availability of unzip... "
  if hash unzip 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  if [ "$USE_HOMEBREW" = "true" ]; then
    echo -n "Checking availability of Homebrew (brew)... "
    if hash brew 2>/dev/null; then
      echo "OK!"
    else
      echo "Missing!"
      SHOULD_EXIT="true"
    fi
  fi

  if [ "$SHOULD_EXIT" = "true" ]; then
    echo "Not installing fnm due to missing dependencies."
    exit 1
  fi
}

ensure_containing_dir_exists() {
  local CONTAINING_DIR
  CONTAINING_DIR="$(dirname "$1")"
  if [ ! -d "$CONTAINING_DIR" ]; then
    echo " >> Creating directory $CONTAINING_DIR"
    mkdir -p "$CONTAINING_DIR"
  fi
}

setup_shell(){
    CURRENT_SHELL="$(basename "$SHELL")"

  if [ "$CURRENT_SHELL" = "zsh" ]; then
    CONF_FILE=${ZDOTDIR:-$HOME}/.zshrc
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Zsh. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # snm'
    echo '  export PATH="'"$INSTALL_DIR"':$PATH"'

    echo '' >>$CONF_FILE
    echo '# snm' >>$CONF_FILE
    echo 'export PATH="'$INSTALL_DIR':$PATH"' >>$CONF_FILE

  elif [ "$CURRENT_SHELL" = "bash" ]; then
    if [ "$OS" = "Darwin" ]; then
      CONF_FILE=$HOME/.profile
    else
      CONF_FILE=$HOME/.bashrc
    fi
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Bash. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # snm'
    echo '  export PATH="'"$INSTALL_DIR"':$PATH"'

    echo '' >>$CONF_FILE
    echo '# snm' >>$CONF_FILE
    echo 'export PATH="'"$INSTALL_DIR"':$PATH"' >>$CONF_FILE

  else
    echo "Could not infer shell type. Please set up manually."
    exit 1
  fi

  echo ""
  echo "In order to apply the changes, open a new terminal or run the following command:"
  echo ""
  echo "  source $CONF_FILE"
}

set_filename
check_dependencies
download_snm
setup_shell