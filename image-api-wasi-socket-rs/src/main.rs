use bytecodec::DecodeExt;
use httpcodec::{
    DecodeOptions, HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode,
};
use image::{ImageFormat, ImageOutputFormat};

use std::io::{Read, Write};
#[cfg(feature = "std")]
use std::net::{Shutdown, TcpListener, TcpStream};
#[cfg(not(feature = "std"))]
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

fn grayscale(image: &[u8]) -> Vec<u8> {
    let detected = image::guess_format(&image);
    let mut buf = vec![];
    if detected.is_err() {
        return buf;
    }
    //println!("process grayscale ...");
    let image_format_detected = detected.unwrap();
    let img = image::load_from_memory(&image).unwrap();
    let filtered = img.grayscale();
    match image_format_detected {
        ImageFormat::Gif => {
            filtered.write_to(&mut buf, ImageOutputFormat::Gif).unwrap();
        }
        _ => {
            filtered.write_to(&mut buf, ImageOutputFormat::Png).unwrap();
        }
    };
    return buf;
}

fn handle_http(req: Request<Vec<u8>>) -> bytecodec::Result<Response<String>> {
    let image = grayscale(req.body());
    //let res = format!("{}=> {:?}", req.body().len(), image.len());
    let res = base64::encode(&image);
    //let res = req.body().len();
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        res,
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

    let body_decoder = httpcodec::BodyDecoder::<bytecodec::bytes::RemainingBytesDecoder>::default();

    // According to https://github.com/sile/httpcodec/blob/master/src/message.rs#L30
    // For processing large image, set this option for enlarging the max_bytes
    // There is a bug in httpcodec, it will not process large image correctly
    let option = DecodeOptions {
        max_start_line_size: 0xFFFF,
        max_header_size: 0xFFFF,
    };
    let mut decoder = RequestDecoder::<
        httpcodec::BodyDecoder<bytecodec::bytes::RemainingBytesDecoder>,
    >::with_options(body_decoder, option);

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_1,
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
