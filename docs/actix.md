# On Actix

So much more [doc](https://actix.rs/docs/)! Very welcoming! And [plentiful examples](https://github.com/actix/examples).

Reloading seems like [a bit of a chore](https://actix.rs/docs/autoreload/). Maybe compile times for a realistic app aren't too bad?

The design of the framework appears, at first glance, very close to "paving the cow paths" of how crates.io is structured. Could be worse!

(In defense of Nickel, Actix uses a very similar concept for passing global state to request handlers)

## Reading the docs

Actix creates a new instance of whatever state is passed for the server _per HTTP server thread_. If you need to share something globally between listeners on a physical host, it has to be shared in an `Arc` cell.

> Request handling happens in two stages. First the handler object is called, returning any object that implements the  [Responder](https://actix.rs/actix-web/actix_web/trait.Responder.html#foreign-impls)  trait. Then, respond_to() is called on the returned object, converting itself to a AsyncResult or Error.

Pretty cool idea!

A thing I’ll need to keep in mind: Actix has ambitions to permit async execution, but Rust is not all the way there as a community/technology yet. So weirdness may ensue, and using patterns/idioms that avoid that will be a handy way to maximize happiness along the way.

Type-ful web programming sounds promising. Lots of guessing/playing computer to eliminate this way. Goodbye key/value mappings of request path/query body/form ata, hello [concrete structured data](https://actix.rs/docs/extractors/)?
