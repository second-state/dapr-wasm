use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
use hyper::header::*;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;
use chrono::prelude::*;
use serde_json::json;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn classify(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    let model_data: &[u8] = include_bytes!("models/mobilenet_v1_1.0_224/mobilenet_v1_1.0_224_quant.tflite");
    let labels = include_str!("models/mobilenet_v1_1.0_224/labels_mobilenet_quant_v1_224.txt");
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /classify such as: `curl http://localhost:9006/classify -X POST --data-binary '@grace_hopper.jpg'`",
        ))),

        (&Method::POST, "/classify") => {
            let headers = req.headers().to_owned();
            let mut ip = "0.0.0.0";
            if headers.contains_key(REFERER) {
                ip = headers.get(REFERER).unwrap().to_str().unwrap();
            } else if headers.contains_key("REMOTE_ADDR") {
                ip = headers.get("REMOTE_ADDR").unwrap().to_str().unwrap();
            }
            println!("IP is {}", ip);

            let buf = hyper::body::to_bytes(req.into_body()).await?;
            let flat_img = wasmedge_tensorflow_interface::load_jpg_image_to_rgb8(&buf, 224, 224);

            let mut session = wasmedge_tensorflow_interface::Session::new(&model_data, wasmedge_tensorflow_interface::ModelType::TensorFlowLite);
            session.add_input("input", &flat_img, &[1, 224, 224, 3])
                .run();
            let res_vec: Vec<u8> = session.get_output("MobilenetV1/Predictions/Reshape_1");

            let mut i = 0;
            let mut max_index: i32 = -1;
            let mut max_value: u8 = 0;
            while i < res_vec.len() {
                let cur = res_vec[i];
                if cur > max_value {
                    max_value = cur;
                    max_index = i as i32;
                }
                i += 1;
            }

            let mut label_lines = labels.lines();
            for _i in 0..max_index {
                label_lines.next();
            }
            let class_name = label_lines.next().unwrap().to_string();
            println!("result: {}", class_name);

            // Connect to local sidecar
            let client = dapr::Dapr::new(3504);
            let ts = Utc::now().timestamp_millis();

            let kvs = json!({ 
                "event_ts": ts, 
                "op_type": "classify", 
                "input_size": buf.len() 
            });
            client.invoke_service("events-service", "create_event", kvs).await?;

            let kvs = json!([{ 
                "key": ip, "value": ts
            }]);
            println!("KVS is {}", serde_json::to_string(&kvs)?);
            client.save_state("statestore", kvs).await?;

            Ok(Response::new(Body::from(format!("{} is detected with {}/255 confidence", class_name, max_value))))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9006));
    let make_svc = make_service_fn(|_| {
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                classify(req)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())

    /*
    let addr = SocketAddr::from(([0, 0, 0, 0], 9006));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(classify)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
    */
}
