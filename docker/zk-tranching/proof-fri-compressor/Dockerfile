# Will work locally only after prior universal setup key download

FROM debian:bookworm-slim as builder

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y curl clang openssl libssl-dev gcc g++ \
    pkg-config build-essential libclang-dev ssh git && \
    rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y && \
    rustup install nightly-2023-08-21 && \
    rustup default nightly-2023-08-21

ARG SSH_KEY

RUN mkdir -p /root/.ssh/ && \
    echo "$SSH_KEY" > /root/.ssh/id_rsa && \
    chmod -R 600 /root/.ssh/ && \
    ssh-keyscan -t rsa github.com >> ~/.ssh/known_hosts

RUN git config --global url."ssh://git@github.com/".insteadOf https://github.com/

WORKDIR /usr/src/micro
COPY . .

RUN cd prover && cargo build --release --bin micro_proof_fri_compressor

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y curl libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

# copy VK required for proof wrapping
COPY prover/vk_setup_data_generator_server_fri/data/ /prover/vk_setup_data_generator_server_fri/data/

# copy universal setup key required for proof compression
COPY setup_2\^26.key /setup_2\^26.key

COPY --from=builder /usr/src/micro/prover/target/release/micro_proof_fri_compressor /usr/bin/

ENTRYPOINT ["micro_proof_fri_compressor"]
