FROM rust as base

LABEL maintainer="Stephane Segning <selastlambou@gmail.com>"
LABEL org.opencontainers.image.description="UI Frontend for Vymalo Projects"

WORKDIR /app

FROM base as builder

COPY ./ ./

RUN cargo build --release

FROM base

# Create non-root user
RUN addgroup -S -g 1001 app && adduser -S app -G app -u 1001

COPY --from=builder --chown=app:app /app/backend-rust-hyper/target/release/backend-rust-hyper /app/backend-rust-hyper

EXPOSE 8000

# Change the user to the non-root user
USER app

CMD ["/app/backend-rust-hyper"]