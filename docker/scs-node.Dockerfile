# build stage: where we create binary
FROM rust:1.80 AS builder

RUN apt update && apt install -y make clang pkg-config libssl-dev protobuf-compiler build-essential git curl llvm make

WORKDIR /scs
COPY . /scs
RUN cargo build --release --features scs

# This is the 2nd stage: a very small image where we copy the scs binary."
FROM docker.io/library/ubuntu:22.04
LABEL description="Docker image for Super Smart Chain." \
	io.parity.image.type="builder" \
	io.parity.image.authors="weimeme@SuperEx" \
	io.parity.image.vendor="SuperEx" 

COPY --from=builder /scs/target/release/scs /usr/local/bin
COPY --from=builder /scs/scripts/validator_node_init.sh /usr/local/bin
COPY --from=builder /scs/scripts/normal_node_init.sh /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/base -d /scs scs && \
	mkdir -p /data /scs/.local/share/scs && \
	chown -R scs:scs /data && \
	ln -s /data /scs/.local/share/scs && \
# Sanity checks
	ldd /usr/local/bin/scs && \
    chmod 777 /usr/local/bin/validator_node_init.sh && \
	chmod 777 /usr/local/bin/normal_node_init.sh && \
	/usr/local/bin/scs --version

USER scs
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/scs", "--database", "auto", "--base-path", "/data" ]
CMD [ "--help" ]
