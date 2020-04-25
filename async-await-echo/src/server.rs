use {
    hyper::{
        service::{make_service_fn, service_fn},
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::net::SocketAddr,
};

async fn serve_rq(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("request uri: {}", req.uri());
    Ok(Response::new(Body::from("hello, world")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr)
        .serve(make_service_fn(|_| {
            async {
                {
                    Ok::<_, hyper::Error>(service_fn(serve_rq))
                }
            }
        }));

    if let Err(e) = serve_future.await {
        eprintln!("server error: {}", e);
    }
}

trait RefMe {
    fn ref_me(&mut self) -> &Self;
}

impl RefMe for i32 {
    fn ref_me(&mut self) -> &Self {
        self
    }
}

#[tokio::main]
async fn _run() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    run_server(addr).await;
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    _run();
    Ok(())
}
