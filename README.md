# wasm-barebones
This project is intended as a minimal example of using Rust to generate WebAssembly that runs in the web browser.
See also [this blog post.](http://secretartofscience.com/wasm-getting-started)

To add wasm a compilation target, use rustup:
~~~
rustup target add wasm32-unknown-unknown
~~~

Then, to compile your file, go:
~~~
rustc --target wasm32-unknown-unknown bones.rs
~~~

In order to load WebAssembly, you need to pass browser [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) checks.
The easiest way to do this is start a local server. You can do this with python3 at the command line by just typing:
~~~
python3 -m http.server
~~~
This works, but it serves the files with cache headers, which can make development confusing.
This project includes a slightly longer Python script which adds no-cache to the built-in http server.
