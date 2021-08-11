go build --tags "tensorflow image" &&
dapr run --app-id image-api-go \
         --app-protocol http \
         --app-port 9003 \
         --dapr-http-port 3501 \
         --log-level debug \
         --components-path ../config \
         ./image-api-go
