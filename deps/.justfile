set dotenv-load

revision := env_var('SDK_GIT_REVISION')

[private]
clone-rust-sdk target revision=revision force="false":
    #!/usr/bin/env sh
    if {{force}} || ! {{path_exists(target)}}; then
        git submodule add --force https://github.com/Zondax/ledger-rust
        git submodule update --init --remote
        git -C ledger-rust checkout {{revision}}
    fi

# Fetches the app SDK, cloning it if necessary and preparing the required symlinks
sdk force="false" revision=revision: (clone-rust-sdk "ledger-rust" revision force)
    just ledger-rust/link-ledger-sdk "./ledger-rust" $(pwd) {{force}}
