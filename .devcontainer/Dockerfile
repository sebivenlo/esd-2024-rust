FROM rust:1.82-slim-bullseye

RUN apt-get update && apt-get install --no-install-recommends -y \
    build-essential \
    curl \
    libssl-dev \
    pkg-config \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace

CMD ["bash"]