#[macro_use] extern crate nickel;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use nickel::status::StatusCode;
use nickel::{ Nickel, JsonBody, HttpRouter, MediaType };

#[derive(Serialize, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
}

fn main() {
    let mut server = Nickel::new();

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
