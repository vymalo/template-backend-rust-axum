FROM rust as base

LABEL maintainer="Stephane Segning <selastlambou@gmail.com>"
LABEL org.opencontainers.image.description="UI Frontend for Vymalo Projects"

WORKDIR /app

FROM base as builder

COPY ./ ./

RUN cargo build --release

FROM debian as dep

RUN apt-get update && apt-get install -y libpq5

# Dependencies for libpq (used by diesel)
RUN mkdir /deps && \
  cp /usr/lib/*-linux-gnu/libpq.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgssapi_*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libunistring.so* /deps && \
  cp /usr/lib/*-linux-gnu/libidn*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libkeyutils.so* /deps && \
  cp /usr/lib/*-linux-gnu/libtasn1.so* /deps && \
  cp /usr/lib/*-linux-gnu/libnettle.so* /deps && \
  cp /usr/lib/*-linux-gnu/libhogweed.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgmp.so* /deps && \
  cp /usr/lib/*-linux-gnu/libffi.so* /deps && \
  cp /usr/lib/*-linux-gnu/libp11-kit.so* /deps && \
  cp /usr/lib/*-linux-gnu/libkrb*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libcom_err.so* /deps && \
  cp /usr/lib/*-linux-gnu/libk5crypto.so* /deps && \
  cp /usr/lib/*-linux-gnu/libsasl2.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgnutls.so* /deps && \
  cp /usr/lib/*-linux-gnu/liblber-*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libldap-*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgcc_s.so* /deps

FROM gcr.io/distroless/base-debian12

LABEL maintainer="Stephane Segning <selastlambou@gmail.com>"
LABEL org.opencontainers.image.description="UI Frontend for Vymalo Projects"

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
COPY --from=dep /deps /usr/lib/

EXPOSE 3000

CMD ["/app/backend"]