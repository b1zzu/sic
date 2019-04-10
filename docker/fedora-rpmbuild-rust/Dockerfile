FROM fedora

RUN dnf install -y rpm-build gcc

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf -o /rust-init.sh \
 && sh /rust-init.sh -y --no-modify-path \
 && rm /rust-init.sh

RUN cargo install cargo-rpm

RUN mkdir /project

WORKDIR /project
