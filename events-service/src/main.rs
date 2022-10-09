use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode, Server};
pub use mysql_async::prelude::*;
pub use mysql_async::*;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    id: Option<i32>,
    event_ts: i64,
    op_type: String,
    input_size: i32,
}

impl Event {
    fn new(
        id: Option<i32>,
        event_ts: i64,
        op_type: String,
        input_size: i32,
    ) -> Self {
        Self {
            id,
            event_ts,
            op_type,
            input_size,
        }
    }
}

async fn handle_request(req: Request<Body>, pool: Pool) -> Result<Response<Body>, anyhow::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "The valid endpoints are /init /create_event /events",
        ))),

        (&Method::GET, "/init") => {
            println!("/init");
            let mut conn = pool.get_conn().await.unwrap();

            "DROP TABLE IF EXISTS image_evts;".ignore(&mut conn).await?;
            "CREATE TABLE image_evts (id INT PRIMARY KEY AUTO_INCREMENT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, event_ts BIGINT, op_type VARCHAR(20), input_size INT);".ignore(&mut conn).await?;

            drop(conn);
            Ok(Response::new(Body::from("{\"status\":true}")))
        }

        (&Method::POST, "/create_event") => {
            println!("/create_event");
            let mut conn = pool.get_conn().await.unwrap();
            println!("GET conn");

            let byte_stream = hyper::body::to_bytes(req).await?;
            let event: Event = serde_json::from_slice(&byte_stream).unwrap();
            println!("Event is {}", serde_json::to_string(&event)?);

            "INSERT INTO image_evts (event_ts, op_type, input_size) VALUES (:event_ts, :op_type, :input_size)"
                .with(params! {
                    "event_ts" => event.event_ts,
                    "op_type" => event.op_type,
                    "input_size" => event.input_size,
                })
                .ignore(&mut conn)
                .await?;
            println!("INSERTed into image_evts");

            drop(conn);
            Ok(Response::new(Body::from("{\"status\":true}")))
        }

        (&Method::GET, "/events") => {
            println!("/events");
            let mut conn = pool.get_conn().await.unwrap();

            let events = "SELECT id, event_ts, op_type, input_size FROM image_evts"
                .with(())
                .map(&mut conn, |(id, event_ts, op_type, input_size)| {
                    Event::new(
                        id,
                        event_ts,
                        op_type,
                        input_size,
                    )},
                ).await?;

            drop(conn);
            Ok(Response::new(Body::from(serde_json::to_string(&events)?)))
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
    println!("App started. Wait for Dapr sidecar to start ...");
    sleep(Duration::from_millis(1500)).await;

    let client = dapr::Dapr::new(3505);
    // let mysql_cred = client.get_secret("local-store", "MYSQL:CRED").await?;
    // let mysql_conn = client.get_secret("local-store", "MYSQL:CONN").await?;
    // println!("MYSQL value is {} {}", mysql_cred, mysql_conn);
    // let db_url = "mysql://".to_string() + &mysql_cred["MYSQL:CRED"].as_str().unwrap() + "@" + &mysql_conn["MYSQL:CONN"].as_str().unwrap();
    let v = client.get_secret("local-store", "DB_URL:MYSQL").await?;
    println!("MYSQL value is {}", v);
    let db_url = v["DB_URL:MYSQL"].as_str().unwrap();
    println!("Connection is {}", db_url);

    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    // The connection pool will have a min of 5 and max of 10 connections.
    let constraints = PoolConstraints::new(5, 10).unwrap();
    let pool_opts = PoolOpts::default().with_constraints(constraints);
    let pool = Pool::new(builder.pool_opts(pool_opts));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9007));
    let make_svc = make_service_fn(|_| {
        let pool = pool.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let pool = pool.clone();
                handle_request(req, pool)
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
