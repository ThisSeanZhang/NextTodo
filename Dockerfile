FROM debian:bookworm-slim
WORKDIR /app
COPY result /app

EXPOSE 52525
CMD ["./nexttodo-server"]
