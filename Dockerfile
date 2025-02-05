FROM ubuntu:22.04 AS env

RUN apt-get update
RUN apt-get install -y gcc g++ python3 curl

RUN useradd -ms /bin/bash coderunner
USER coderunner
WORKDIR /home/coderunner

ENV NVM_DIR=/home/coderunner/.nvm
ENV NODE_VERSION=20.11.1
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
RUN . "$NVM_DIR/nvm.sh" \
    && nvm install ${NODE_VERSION} \
    && nvm alias default ${NODE_VERSION} \
    && nvm use default
ENV PATH="/home/coderunner/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN npm install -g ts-node

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/home/coderunner/.cargo/bin:${PATH}"

FROM rust:1.77 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM env AS runner
COPY --from=builder /usr/local/cargo/bin/code-runner /usr/local/bin/code-runner
EXPOSE 8080
CMD ["code-runner"]
