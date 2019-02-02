# On nickel.rs

The immediate appeal is "Express-like", which bakes in a usable idea of an API to start. The single landing page of docs aren't as good as the others. 

I found the [examples](https://github.com/nickel-org/nickel.rs/blob/master/examples) page and that helped _a lot_.

Nickel uses macro expansions for defining middleware and routers. I think this is syntactic sugar, but it's new territory for me! They do have no-macro examples though!

After skimming over all the examples, Nickel seems pretty comparable to Sinatra in scope and coverage. Rust's type system makes structured handling of JSON more easily graspable, though.

## Everything (?) is a middleware

I'm using `StaticFilesHandler` to figure out how to build composable units here. It's a middleware, which makes sense from prior experience with e.g. Rack. 

All middleware implement `invoke` from the `Middleware` trait. Adding a trait onto one of your own structs is just a method call, so it can access any state in the constructor function for your struct. So it basically looks like:

- write a struct, put some state in it
- write methods for `new` and generating a router
- implement invoke on that stuff?

But maybe its different if you're returning a router?

Also, feels like I'm making a minor hop forward in learning Rust. I'm a) reading types a little bit now (like they say one does in Haskell) and b) looking for truth and examples in sources instead of just in docs and reference material.

## On passing connection-like things

So, I’m trying to write a Rust web thingy with Nickel. I want to be able to pass a DB connection-like thing between various middlewares/routers. Nickel.rs is a little thin on examples, so I’m not entirely sure how to do that.

By comparison, crates.io does it like so:

- every "controller" (close to a router) is passed a custom-to-crates request object
- the database connection is an accessor on that request object
- the database connection management module defines and adds a trait on that custom request object
- that trait looks at the app object on the request to attempt to pull a connection out of the global DB pool

So with crates, you don't even need to worry about shuffling a value through the type system, so much.

## On encapsulation, middlewares, and routers

You can slightly encapsulate a type in a middleware like [this example](https://github.com/nickel-org/nickel.rs/blob/master/examples/integration_testing.rs#L90-L111). Mostly, you're stuck with passing a big struct of application state/configuration to `Nickel::with_data` and pulling it out from each router or middleware.

Maybe this isn't as big a deal with Rust as it would be in Ruby! For example, Diesel models always take a connection parameter, so every controller wouldn't necessarily go tinkering with some other bit of state.