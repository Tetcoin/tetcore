/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
brew install openssl cmake
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
cargo install --git https://github.com/tetcoin/tetcore tetkey
cargo install --git https://github.com/tetcoin/tetcore tetcore
