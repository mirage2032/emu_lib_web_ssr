FROM ubuntu:latest
RUN apt update && apt upgrade -y

# Install Dependencies
RUN apt install -y build-essential libssl-dev libcrypt-dev postgresql
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add wasm32-unknown-unknown
RUN cargo install stylance-cli cargo-leptos wasm-bindgen-cli

# Set Working Directory
WORKDIR /app

# Copy essential files
COPY app ./app
COPY frontend ./frontend
COPY server ./server
COPY public ./public
COPY style ./style
COPY .env ./.env
COPY Cargo.toml ./Cargo.toml
COPY diesel.toml ./diesel.toml
COPY run_stylance.sh ./run_stylance.sh
COPY rust-toolchain.toml ./rust-toolchain.toml

# Build
RUN stylance app
RUN cargo leptos build --release

# Expose Port
EXPOSE 3000

# Run
CMD ["cargo", "run", "--release"]