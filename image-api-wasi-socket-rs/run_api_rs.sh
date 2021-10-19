dapr run --app-id image-api-wasi-socket-rs \
         --app-protocol http \
         --app-port 9005 \
         --dapr-http-port 3502 \
         --components-path ../config \
         --log-level debug \
         ./target/release/image-api-wasi-socket-rs
