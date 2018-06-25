FROM ubuntu:18.04

ENV HOME /home/developer
ENV RUSTUP_HOME ${HOME}/.rustup
ENV CARGO_HOME ${HOME}/.cargo
ENV PATH=${HOME}/.cargo/bin:$PATH

# Install required system libraries
RUN apt-get update \
    && apt-get install -y \
    curl \
    cmake \
    git \
    libasound2 \
    libgtk2.0-0 \
    libssl-dev \
    libxss1 \
    pkg-config \
    sudo \
    upx-ucl \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

# Add a user "developer" with password "password" 
RUN useradd --create-home --shell /bin/bash -g root -G sudo developer \
    && echo 'developer:password' | chpasswd
WORKDIR ${HOME}

# Install rust through rustup
USER developer
RUN curl -o rustup.sh https://sh.rustup.rs -sS \
    && sh rustup.sh -y --no-modify-path \
    && rm -f rustup.sh
USER root

# Install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
    && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list \
    && apt-get update \ 
    && apt-get -y install yarn \
    && rm -rf /var/lib/apt/lists/*

# Install vscode
RUN apt-get update \
    && curl -o vscode.deb -J -L https://vscode-update.azurewebsites.net/1.24.1/linux-deb-x64/stable \
    && apt install -y ./vscode.deb \
    && rm -f vscode.deb \
    # Cleanup after apt
    && rm -rf /var/lib/apt/lists/*
# Install vscode extensions
USER developer
RUN code --install-extension bungcip.better-toml
RUN code --install-extension streetsidesoftware.code-spell-checker
RUN code --install-extension msjsdiag.debugger-for-chrome
RUN code --install-extension PKief.material-icon-theme
RUN code --install-extension AdamCaviness.theme-monokai-dark-soda
RUN code --install-extension esbenp.prettier-vscode
RUN code --install-extension rust-lang.rust
RUN code --install-extension robinbentley.sass-indented
RUN code --install-extension mrmlnc.vscode-scss
RUN code --install-extension eg2.tslint
USER root

# Install vendored crates
USER developer
RUN cargo install cargo-local-registry \
    && git clone https://github.com/davidMcneil/crates-vendor.git \
    && cd crates-vendor \
    && cargo build \
    && cargo local-registry --sync Cargo.lock ${HOME}/crates-registry
# && cargo uninstall cargo-local-registry \
# && cd .. \
# && rm -rf crates-vendor
RUN echo "[source.crates-io]" >> .cargo/config \
    && echo "registry = 'https://github.com/rust-lang/crates.io-index'" >> .cargo/config \
    && echo "replace-with = 'local-registry'" >> .cargo/config \
    && echo "" >> .cargo/config \
    && echo "[source.local-registry]" >> .cargo/config \
    && echo "local-registry = '/home/developer/crates-registry'" >> .cargo/config
USER root

### Install vendored npm


WORKDIR ${HOME}/project

USER developer
