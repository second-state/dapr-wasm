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
    id: i32,
    ts: String,
    op_type: i32, // 1: grayscale; 2: classify
    input_size: i32,
}

impl Event {
    fn new(
        id: i32,
        ts: String,
        op_type: i32,
        input_size: i32,
    ) -> Self {
        Self {
            id,
            ts,
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
            let mut conn = pool.get_conn().await.unwrap();
            "DROP TABLE IF EXISTS events;".ignore(&mut conn).await?;
            "CREATE TABLE events (id INT AUTO_INCREMENT, ts DATETIME, op_type INT, input_size INT);".ignore(&mut conn).await?;
            drop(conn);
            Ok(Response::new(Body::from("{\"status\":true}")))
        }

        (&Method::POST, "/create_event") => {
            let mut conn = pool.get_conn().await.unwrap();

            let byte_stream = hyper::body::to_bytes(req).await?;
            let event: Event = serde_json::from_slice(&byte_stream).unwrap();

            "INSERT INTO events (op_type, input_size) VALUES (:op_type, :input_size)"
                .with(params! {
                    "op_type" => event.op_type,
                    "input_size" => event.input_size,
                })
                .ignore(&mut conn)
                .await?;

            drop(conn);
            Ok(Response::new(Body::from("{\"status\":true}")))
        }

        (&Method::GET, "/events") => {
            let mut conn = pool.get_conn().await.unwrap();

            let events = "SELECT * FROM events"
                .with(())
                .map(&mut conn, |(id, ts, op_type, input_size)| {
                    Event::new(
                        id,
                        ts,
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
    let v = client.get_secret("local-store", "DB_URL:MYSQL").await?;
    println!("DB_URL:MYSQL value is {}", v);
    let db_url = v["DB_URL:MYSQL"].to_string();

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