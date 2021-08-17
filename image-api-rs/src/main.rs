use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::Ipv4Addr;
use std::process::{Command, Stdio};
use warp::Filter;
use wedge::value::*;

pub fn image_process(buf: &Vec<u8>) -> String {
    let mut child = Command::new("./lib/wasmedge-tensorflow-lite")
        .arg("./lib/classify.so")
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
    let ans = String::from_utf8_lossy(&output.stdout);
    ans.to_string()
}

pub fn wedge_image_process(buf: &Vec<u8>) -> String {
    let module_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib/grayscale_lib_bg.wasm");

    println!("module_path: {:?}", module_path);
    let config = wedge::Config::default();
    let module = wedge::Module::load_from_file(&module_path, &config).unwrap();
    let mut vm = wedge::Vm::create(&module, &config).unwrap();

    let params = buf
        .iter()
        .map(|&x| Value::I32(x as i32))
        .collect::<Vec<Value>>();
    let results = vm.run("grayscale", &params).unwrap();
    println!("results: {:?}", results);
    assert_eq!(results.len(), 1);
    let result = results[0].as_i32().unwrap();
    return String::from("");
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
            println!("result: {}", res);
            Ok(Box::new(res))
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

fn main_prev() {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let _port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 9004,
    };

    run_server(_port);
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let path = std::env::args().nth(1).expect("expect image");
    let image = get_file_as_byte_vec(&path);
    let res = wedge_image_process(&image);
    println!("res: {}", res);
}
