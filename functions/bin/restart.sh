dapr list 
dapr stop image-api-go 
dapr stop image-api-rs
dapr stop go-web-port 
dapr stop image-api-wasi-socket-rs


pushd web-port 
nohup ./run_web.sh  > nohup.log &
popd 

pushd image-api-rs
nohup ./run_api_rs.sh > nohup.log &
popd

pushd image-api-go 
nohup ./run_api_go.sh > nohup.log &
popd

pushd image-api-wasi-socket-rs
nohup ./run_api_wasi_socket_rs.sh > nohup.log & 
popd

