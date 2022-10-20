use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
use hyper::header::*;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;
use chrono::prelude::*;
use serde_json::json;
use image::{ImageFormat, ImageOutputFormat};

async fn grayscale(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /grayscale such as: `curl http://localhost:9005/grayscale -X POST --data-binary '@my_img.png'`",
        ))),

        // CORS Options
        (&Method::OPTIONS, "/grayscale") => Ok(response_build(&String::from(""), "text/html")),

        (&Method::POST, "/grayscale") => {
            let headers = req.headers().to_owned();
            let mut ip = "0.0.0.0";
            if headers.contains_key(REFERER) {
                ip = headers.get(REFERER).unwrap().to_str().unwrap();
            } else if headers.contains_key("REMOTE_ADDR") {
                ip = headers.get("REMOTE_ADDR").unwrap().to_str().unwrap();
            }
            println!("IP is {}", ip);

            let image_data = hyper::body::to_bytes(req.into_body()).await?;
            let detected = image::guess_format(&image_data);
            let mut buf = vec![];
            if detected.is_err() {
                return Ok(Response::new(Body::from("Unknown image format")));
            }
            println!("process grayscale ...");
            let image_format_detected = detected.unwrap();
            let img = image::load_from_memory(&image_data).unwrap();
            let filtered = img.grayscale();
            match image_format_detected {
                ImageFormat::Gif => {
                    filtered.write_to(&mut buf, ImageOutputFormat::Gif).unwrap();
                }
                _ => {
                    filtered.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
                }
            };

            // Connect to the attached sidecar
            let client = dapr::Dapr::new(3503);
            let ts = Utc::now().timestamp_millis();

            let kvs = json!({
                "event_ts": ts,
                "op_type": "grayscale",
                "input_size": image_data.len()
            });
            client.invoke_service("events-service", "create_event", kvs).await?;

            let kvs = json!([{
                "key": ip, "value": ts
            }]);
            println!("KVS is {}", serde_json::to_string(&kvs)?);
            client.save_state("statestore", kvs).await?;

            let res = base64::encode(&buf);
            Ok(response_build(&res, "image/png"))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// CORS headers
fn response_build(body: &String, content_type: &str) -> Response<Body> {
    Response::builder()
        .header("Content-Type", content_type)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "api,Keep-Alive,User-Agent,Content-Type")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 9005));
    let make_svc = make_service_fn(|_| {
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                grayscale(req)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
