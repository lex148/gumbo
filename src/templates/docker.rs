use super::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./Dockerfile");
    let mut file = File::create(path)?;
    file.write_all(CODE.trim().as_bytes())?;
    Ok(())
}

static CODE: &str = r#"
FROM clux/muslrust:1.77.2-stable AS chef
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

# Build application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin server
RUN strip /app/target/x86_64-unknown-linux-musl/release/server 

# Build runtime image
FROM scratch AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /app/server
USER 1001
ENV RUST_LOG="info,sqlx=warn"
ENTRYPOINT ["/app/server"]
#CMD []
"#;