FROM rust:1.77 as builder

# Install ProtoC
ENV PROTOC_ZIP=protoc-26.0-linux-x86_64.zip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v26.0/${PROTOC_ZIP}
RUN unzip -o ${PROTOC_ZIP} -d ./proto
RUN chmod 755 -R ./proto/bin
ENV BASE=/usr

RUN cp ./proto/bin/protoc ${BASE}/bin/
RUN cp -R ./proto/include/* ${BASE}/include/

WORKDIR /app
COPY . .

RUN cargo build --release


FROM debian:bookworm-slim

ENV AOBA_ARKALIS_URL=https://api.arkalis.org

RUN apt-get update \
    && apt-get install openssl curl -y

WORKDIR /app
COPY --from=builder /app/target/release/aoba .
COPY Rocket.toml .

EXPOSE 8001

RUN mkdir uploads

ENTRYPOINT ["./aoba"]
