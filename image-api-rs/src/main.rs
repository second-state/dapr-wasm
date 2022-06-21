use std::env;
use std::fs;
use std::io::Write;
use std::net::Ipv4Addr;
use std::process::{Command, Stdio};
use warp::{http::Response, Filter};
use wasmedge_sys::*;
use std::path::Path;
use wasmedge_bindgen_host::*;


/* This is no use now, replaced with image_process_wasmedge_sys */
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
    let res = output.stdout;
    println!("res: {:?}", res);
    res
}

pub fn image_process_wasmedge_sys(buf: &Vec<u8>) -> String {
    let mut config = Config::create().unwrap();
	config.wasi(true);

	let mut vm = Vm::create(Some(config), None).unwrap();
	let wasm_path = Path::new("./lib/grayscale_lib_origin.wasm");
	let _ = vm.load_wasm_from_file(wasm_path);
	let _ = vm.validate();

	let mut bg = Bindgen::new(vm);

    let image_str = buf.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(",");
    let params = vec![Param::String(image_str)];
    match bg.run_wasm("grayscale_str", params) {
        Ok(res) => {
            let output = res.unwrap().pop().unwrap().downcast::<String>().unwrap();
            //println!("Success: {:?}", &output);
            return *output;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    return "".to_string();
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
            let v: Vec<u8> = bytes.iter().map(|&x| x).collect();
            println!("len {}", v.len());
            let res = image_process_wasmedge_sys(&v);
            //println!("res: {:?}", res);
            let _encoded = base64::encode(&res);
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

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let source_file = &args[1];
            let to_file = &args[2];
            let image = fs::read(source_file).unwrap();
            let res = image_process(&image);
            println!("source: {}", source_file);
            println!("to: {}", to_file);
            //println!("image: {:?}", image);
            fs::write(to_file, &res).unwrap();
            println!("len: {} => {}", image.len(), res.len());
        }
        _ => {
            let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
            let _port: u16 = match env::var(port_key) {
                Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
                Err(_) => 9004,
            };

            run_server(_port);
        }
    }
}
