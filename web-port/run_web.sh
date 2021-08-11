go build &&

dapr run --app-id go-web-port \
         --app-protocol http \
         --app-port 8080 \
         --dapr-http-port 3500 \
         --components-path ../config \
         --log-level debug \
         ./web-port