# Use the official Rust image as the base image
FROM rust:1.67 as builder

RUN USER=root cargo new --bin rust-url-shortener
WORKDIR ./rust-url-shortener
COPY ./Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copy the application source code to the container
ADD src ./src

# Build the application
RUN rm ./target/release/deps/rust_url_shortener*
RUN cargo build --release

# Create a new minimal image
FROM debian:buster-slim
ARG APP=/usr/src/app
ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust-url-shortener/target/release/rust-url-shortener ${APP}/rust-url-shortener

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

# Expose the port(s) that your application listens on
EXPOSE 8000

# Run the application
CMD ["./rust-url-shortener"]
