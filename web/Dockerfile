FROM alpine:latest as builder

# Install Dependencies
RUN apk add --no-cache curl gcc musl-dev openssl-dev libpq-dev \
    pkgconfig perl make g++ dart-sass-js libcrypto3 openssl-libs-static postgresql-dev
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH=/root/.cargo/bin:$PATH

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked
RUN cargo install stylance-cli wasm-bindgen-cli

# Set Working Directory
WORKDIR /web

# Copy essential files
COPY . .

# Build Style
RUN stylance app

# Type erasure flag to save memory
ENV RUSTFLAGS="--cfg erase_components"

# Build Leptos
RUN cargo leptos build --release -vv

FROM alpine:latest as runtime

# Install Dependencies
RUN apk add --no-cache openssl ca-certificates

# Set Working Directory
WORKDIR /web

# Copy built files from builder stage
COPY --from=builder /web/target/release/server /web/
COPY --from=builder /web/target/site /web/site

# Set environment variables
ENV LEPTOS_SITE_ROOT="site"

# Set the command to run the application
CMD ["/web/server"]