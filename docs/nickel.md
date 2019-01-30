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