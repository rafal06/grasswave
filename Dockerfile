FROM rust:1.66 as builder

# Create a dummy project used to grab dependencies
RUN cargo new --bin /grasswave
WORKDIR /grasswave

# Copy only the Cargo files
COPY ./Cargo.* ./

# Build the dependencies
# and delete the dummy project
RUN cargo build --release \
    && rm ./src/* \
    && rm ./target/release/deps/grasswave*

# Copy the complete project
COPY . .
# And build it
RUN cargo build --release

#______________________Runtime_image_______________________
FROM debian:bullseye-slim

WORKDIR /grasswave
COPY --from=builder /grasswave/target/release/grasswave ./
COPY --from=builder /grasswave/static    ./static
COPY --from=builder /grasswave/templates ./templates

EXPOSE 7000
VOLUME ["/data"]

ENV DOCKER 1
ENV ROCKET_ADDRESS 0.0.0.0

CMD ["./grasswave"]
