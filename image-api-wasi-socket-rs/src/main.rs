#![deny(warnings)]
#![warn(rust_2018_idioms)]
use hyper::{body::HttpBody as _, Client};

use std::net::SocketAddr;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

use image::{ImageFormat, ImageOutputFormat};

async fn grayscale(req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /grayscale such as: `curl http://localhost:9005/image -X POST --data-binary '@my_img.png'`",
        ))),

        (&Method::POST, "/image") => {
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
            Ok(Response::new(Body::from(res)))
        }

        (&Method::GET, "/v1.0/state/statestore") => Ok(Response::new(Body::from(
            "Try saving state by POSTing data to /v1.0/state/statestore such as: `curl -X POST -H \"Content-Type: application/json\" -d '[{ \"key\": \"name\", \"value\": \"Bruce\"}]' http://localhost:9005/v1.0/state/statestore`",
        ))),

        (&Method::POST, "/v1.0/state/statestore") => {
            let url_str = "http://localhost:3503/v1.0/state/statestore/";
            const POST_BODY: &[u8] = br#"[{ "key": "name", "value": "Bruce"}]"#;
            println!("\nPOST and get result as string: {}", url_str);
            println!("with a POST body: {:?}", String::from_utf8_lossy(POST_BODY));
            let url = url_str.parse::<hyper::Uri>().unwrap();
            post_url_return_str(url, POST_BODY).await?;
            Ok(Response::new(Body::from("Successfull post")))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}


async fn post_url_return_str (url: hyper::Uri, post_body: &'static [u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Length", post_body.len())
        .header("Content-Type", "application/json")
        .body(Body::from(post_body))?;
    let mut res = client.request(req).await?;

    let mut resp_data = Vec::new();
    while let Some(next) = res.data().await {
        let chunk = next?;
        resp_data.extend_from_slice(&chunk);
    }
    println!("{}", String::from_utf8_lossy(&resp_data));

    Ok(())
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
