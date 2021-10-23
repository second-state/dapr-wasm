use std::env;
use std::io::Write;
use std::net::Ipv4Addr;
use std::process::{Command, Stdio};
use warp::{http::Response, Filter};

pub fn image_process(buf: &Vec<u8>) -> Vec<u8> {
    let mut child = Command::new("wasmedge")
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
    println!("len: {:?} => {:?}", buf.len(), output.stdout.len());
    output.stdout
}

#[tokio::main]
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
            let encoded = base64::encode(&res);
            println!("result len: {:?}", res.len());
            Response::builder()
                .header("content-type", "image/png")
                .body(encoded)
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
