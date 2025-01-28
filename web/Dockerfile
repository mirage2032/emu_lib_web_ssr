FROM ubuntu:latest
RUN apt update && apt upgrade -y

# Install Dependencies
RUN apt install -y curl gcc libssl-dev libcrypt-dev libpq-dev pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add wasm32-unknown-unknown
RUN cargo install stylance-cli cargo-leptos wasm-bindgen-cli

# Set Working Directory
WORKDIR /app

# Copy essential files
COPY . .

# Build
RUN stylance app
RUN cargo leptos build --release

# Expose Port
EXPOSE 3000

# Run
CMD ["cargo", "leptos", "serve", "--release"]