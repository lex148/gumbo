use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![
        Change::new("./Dockerfile", CODE)?,
        Change::new("./.dockerignore", IGNORE)?,
    ])
}

static IGNORE: &str = r#"
# Ignore everything
*

# Allowed
!/src
!/Cargo.lock
!/Cargo.toml
!/build.rs
"#;

static CODE: &str = r#"
FROM clux/muslrust:amd64-1.84.1-stable-2025-02-18 AS chef
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \ 
  && chmod a+xr tailwindcss-linux-x64 \
  && mv tailwindcss-linux-x64 /usr/bin/tailwindcss

RUN cargo install cargo-chef

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .

# Update the permissions so that the app user can access the assets
RUN chmod -R a+r /app/src/assets
RUN find /app/src/assets -type d -exec chmod a+rx {} \;

# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl --bin server
RUN strip /app/target/x86_64-unknown-linux-musl/release/server


# Build runtime image
FROM scratch AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /app/server
COPY --from=builder /app/src/assets /src/assets
USER 1001
ENV RUST_LOG="info,sqlx=warn"
ENV HOST="0.0.0.0"
ENTRYPOINT ["/app/server"]
"#;
