extern crate futures;
extern crate hyper;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate tokio_io_pool;
extern crate uri;

#[macro_use]
extern crate jsonapi;

extern crate postgres;

use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};

use postgres::rows::Row;
use postgres::types::{FromSql, ToSql};
use postgres::{Connection, TlsMode};

use futures::future;
use std::collections::HashMap;
use std::env;

use jsonapi::model::*;
use jsonapi::query::*;

type BoxedResponse = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    id: i32,
    title: String,
    description: String,
    rating: Rating,
}
jsonapi_model!(Content; "content"; has one rating);

#[derive(Serialize, Deserialize, Debug)]
struct Rating {
    id: i32,
    name: String,
    description: String,
}
jsonapi_model!(Rating; "rating");

fn load_movies(query_str: Option<&str>) -> String {

    let conn_string = env::var("CONN_STRING")
        .expect("Expected environment variable `CONN_STRING`.");

    let conn = Connection::connect(
        conn_string,
        TlsMode::None,
    )
    .unwrap();
    let query = Query::from_params(query_str.unwrap_or("!"));
    print!("\n\nQuery: {:?}\nQuery String: {:?}\n", query, query_str);
    let rows = &conn.query("SELECT c.content_id, c.title, c.description, r.content_rating_id, r.name, r.description FROM contents c LEFT JOIN content_ratings r ON c.content_rating = r.content_rating_id LIMIT 10", &[]).unwrap();

    let mut contents: Vec<Content> = vec![];
    for row in rows {
        let rating = Rating {
            id: row.get(3),
            name: row.get(4),
            description: row.get(5),
        };
        let content = Content {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            rating: rating,
        };
        contents.push(content);
    }
    let (resources, included) = vec_to_jsonapi_resources(contents);

    //Need to find a generic way of doing this
    let filtered: Option<Vec<Resource>> = match (query.include, included) {
        (Some(include), Some(included)) => {
            let mut filtered: Vec<Resource> = vec![];
            for resource in included {
                print!("--> Resource Id: {:?}",resource._type);
                if include.contains(&resource._type) {
                    filtered.push(resource);
                }
            }
            Some(filtered)
        }
        _ => None,
    };

    let doc = JsonApiDocument {
        data: Some(PrimaryData::Multiple(resources)),
        included: filtered,
        ..Default::default()
    };
    let json = serde_json::to_string(&doc).unwrap();
    json
}

fn app(req: Request<Body>) -> BoxedResponse {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from(load_movies(req.uri().query()));
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

fn main() {
    let new_service = move || service_fn(move |request| app(request));

    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    tokio_io_pool::run(server);
}
