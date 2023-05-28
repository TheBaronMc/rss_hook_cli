FROM rust:latest

WORKDIR /

# INSTALL DEPENDENCIES
RUN apt update

# CLONE REPO
RUN git clone https://github.com/TheBaronMc/rss_hook_cli.git

# COMPILE && INSTALL
WORKDIR /rss_hook_cli
RUN cargo test -- --test-threads=1
RUN cargo install --path .

# DELETE SOURCES
WORKDIR /
RUN rm -r /rss_hook_cli

ENTRYPOINT ["rss_hook_cli"]
CMD ["--help"]
