# wasm-barebones
This project is intended as a minimal example of using Rust to generate WebAssembly that runs in the web browser.
There are much better ways to do this if you are more concerned with the end result than with how these technologies work.
In particular, if you haven't already looked at [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/), you should probably start there.
This is intended as a demo of the underlying technologies and would need a lot of hardening to be solution worthy of production.
See also [this blog post.](http://secretartofscience.com/wasm-getting-started)

To add wasm as a compilation target, use rustup:
~~~
rustup target add wasm32-unknown-unknown
~~~

Then, to compile your file, go:
~~~
rustc --target wasm32-unknown-unknown bones.rs
~~~
If this works, it'll write a file called bones.wasm in the current directory.
This command is also in `build.sh`.

In order to load WebAssembly, you need to pass browser [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) checks.
The easiest way to do this is to start a local server. You can do this with python3 at the command line by typing:
~~~
python3 -m http.server
~~~
This works, but it serves the files with cache headers, which can make development confusing.
This project includes a slightly longer Python script under `serve.py` which adds `no-cache` to the headers sent by the built-in http server.
