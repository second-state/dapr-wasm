use std::net::SocketAddr;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;
use serde_json::json;
use image::{ImageFormat, ImageOutputFormat};

async fn grayscale(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /grayscale such as: `curl http://localhost:9005/grayscale -X POST --data-binary '@my_img.png'`",
        ))),

        (&Method::POST, "/grayscale") => {
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
            
            let res = base64::encode(&buf);
            let response = Response::builder()
                .header("Content-Type", "image/png")
                .body(Body::from(res))
                .unwrap();

            // let client = dapr::Dapr::new(3503);
            let client = dapr::Dapr::new(3505);
            let kvs = json!({ "op_type": 1, "input_size": image_data.len() });
            client.invoke_service("events-service", "create_event", kvs).await?;

            Ok(response)
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
    let addr = SocketAddr::from(([0, 0, 0, 0], 9005));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(grayscale)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
