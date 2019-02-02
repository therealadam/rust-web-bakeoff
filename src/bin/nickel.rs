#[macro_use] extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use nickel::status::StatusCode;
use nickel::{ Nickel, JsonBody, HttpRouter, MediaType };
use std::sync::atomic::{ AtomicUsize, Ordering::Relaxed };
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize)]
struct Count {
    n: usize,
}

fn counter_router() -> nickel::Router<ServerData> {
    let mut router = Nickel::router();

    router.get("/api/count", middleware! { |_req, res| <ServerData>
        res.data().incr_count();

        let count = Count { n: res.data().get_count() };

        serde_json::to_value(count).map_err(|e| (StatusCode::InternalServerError, e))
    });

    router
}

struct ServerData {
    count: AtomicUsize,
}

impl ServerData {
    fn get_count(&self) -> usize {
        self.count.load(Relaxed)
    }

    fn incr_count(&self) -> usize {
        self.count.fetch_add(1, Relaxed)
    }
}

fn main() {
    let server_data = ServerData {
        count: AtomicUsize::new(0)
    };
    let mut server = Nickel::with_data(server_data);
    server.utilize(counter_router());

    let visits = AtomicUsize::new(0);
    let shared_visits = Arc::new(visits);

    let visits_clone = shared_visits.clone();
    server.get("/counts/a", middleware! {
      format!("{}", visits_clone.fetch_add(1, Relaxed))
    });

    server.get("/counts/b", middleware! {
      format!("{}", shared_visits.fetch_add(1, Relaxed))
    });

    server.post("/api/hello", middleware! { |request, response|
      let person = try_with!(response, {
      request.json_as::<Person>().map_err(|e| (StatusCode::BadRequest, e))
      });

      format!("Hello {} {}", person.first_name, person.last_name)
    });

    server.get("/api/hello/:first/:last", middleware! { |req|
      let first_name = req.param("first").unwrap();
      let last_name = req.param("last").unwrap();

      let person = Person {
        first_name: first_name.to_string(),
        last_name: last_name.to_string()
      };

      serde_json::to_value(person).map_err(|e| (StatusCode::InternalServerError, e))
    });

    server.get("/raw", middleware! { |_, mut response|
        response.set(MediaType::Json);

        r#"{ "foo": "bar"}"#
    });

    server.get("/hello", middleware! { |_req, _res|
        "It's not pronounced doink!"
    });

    server.listen("0.0.0.0:3000").unwrap();
}
