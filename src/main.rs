use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};

mod utils;

fn http_error(code: u16) -> Response<Body> {
    return Response::builder()
        .status(422)
        .header("Content-Type", "text/html")
        .body(
            vec!["<img src=https://http.cat/", &code.to_string(), ">"]
                .join("")
                .into(),
        )
        .unwrap();
}

async fn on_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // root
    let pwd = env::current_dir().unwrap();
    let root = Path::new(&pwd);
    // path
    let request_path =
        PathBuf::from_str(&vec![".", (*_req.uri().to_string()).into()].join("")).unwrap();
    let path_asbuf = Path::new("./").join(root).join(Path::new(&request_path));
    let path = Path::new(&path_asbuf);

    if request_path
        .components()
        .into_iter()
        .any(|x| x == Component::ParentDir)
    {
        return Ok(Response::new("Invalid request".into()));
    }

    let test = fs::metadata(path);
    if test.is_err() {
        let err: u16 = 500;
        return Ok(http_error(err));
    } else {
        let metadata = test.unwrap();
        if metadata.is_file() {
            let file = fs::read(path).unwrap();
            let response = Response::builder()
                .status(200)
                .body(Body::from(file))
                .unwrap();
            return Ok(response);
        } else if metadata.is_dir() {
            // Directory listing
            let dir = fs::read_dir(path).unwrap();
            let ents: Vec<String> = dir
                .map(|x| x.unwrap().file_name().to_str().unwrap().into())
                .collect();
            let response = Response::builder()
                .status(200)
                .body(Body::from(
                    ents.iter()
                        .map(|x| {
                            vec![
                                "<a href=",
                                Path::new("/").join(&request_path).join(x).to_str().unwrap(),
                                ">",
                                x,
                                "</a><br>",
                            ]
                            .join("")
                        })
                        .collect::<Vec<String>>()
                        .join(""),
                ))
                .unwrap();
            return Ok(response);
        } else {
            return Ok(http_error(404));
        }
    }
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let port_env: u16 = utils::str::stror(
        //
        env::var("PORT").unwrap(), //
        "3000".into(),
    )
    .unwrap()
    .parse::<u16>()
    .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port_env));
    let server = Server::bind(&addr).serve(
        //
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(on_request)) }),
    );

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
