use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
#[cfg(feature = "std")]
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process::{Command, Stdio};
#[cfg(not(feature = "std"))]
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

pub fn image_process(buf: &[u8]) -> Vec<u8> {
    let mut child = Command::new("wasmedge-tensorflow-lite")
        .arg("./lib/grayscale.wasm")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");
    {
        // limited borrow of stdin
        let stdin = child.stdin.as_mut().expect("failed to get stdin");
        stdin.write_all(buf).expect("failed to write to stdin");
    }
    let output = child.wait_with_output().expect("failed to wait on child");
    output.stdout
}

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    let result = image_process(req.body().as_bytes());
    println!("{:?}", result);
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        format!("echo: {}", req.body()),
    ))
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or(9005.to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    loop {
        let _ = handle_client(listener.accept()?.0);
    }
}

/* #[tokio::main]
pub async fn run_server(port: u16) {
    pretty_env_logger::init();

    let home = warp::get().map(warp::reply);

    // dir already require
    let image = warp::post()
        .and(warp::path("api"))
        .and(warp::path("image"))
        .and(warp::body::bytes())
        .map(|bytes: bytes::Bytes| {
            //println!("bytes = {:?}", bytes);
            let v: Vec<u8> = bytes.iter().map(|&x| x).collect();
            println!("len {}", v.len());
            let res = image_process(&v);
            println!("result len: {:?}", res.len());
            Response::builder()
                .header("content-type", "image/png")
                .body(res)
        });

    let routes = home.or(image);
    let routes = routes.with(warp::cors().allow_any_origin());

    let log = warp::log("dapr_wasm");
    let routes = routes.with(log);
    println!("listen to : {} ...", port);
    warp::serve(routes).run((Ipv4Addr::UNSPECIFIED, port)).await
}

/* async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(warp::reply::json(&format!("{:?}", err)))
}
 */
fn main() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let _port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 9004,
    };

    run_server(_port);
}
 */
