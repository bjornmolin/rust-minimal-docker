# Create minimal docker images with Rust

Run Rust applications in minimal Docker images. 

## Less is more - compile rust with musl target

I do often base my docker images on Alpine images. In order to do that with my Rust project, I have to compile it with musl. The Rust compiler itself will not yet work well in Alpine so the trick is to cross compile with target `x86_64-unknown-linux-musl`. I was very happy to find that I can base my builder on the `clux/muslrust:stable` image.

## Less is more - Alpine is so bloated :)

It can be practical to have a working shell and stuff that You usually have in an operating system. However, when You have built a Rust binary with musl target it is possibly to base the image on `scratch`. What is `scratch`? It is basicly a blank image. 

## Docker tricks

There are some tricks with Docker in order to decrease build time i.e. use cached stages. In this case I do not like cargo to download and build all dependecies on every build. The trick is to build a minimal project with the same `Cargo.toml` and `Cargo.lock` files. The downloading and building of dependencies will only be done when the Cargo files is modified. In fact this is a little bit tricky to do with rust and cargo. The main.rs file has to be present in order to run the first minimal `cargo build`. However, if we ADD it before the first build and replace it before the second build we will invalidate the cached stage in docker build. Cargo is obviously not designed to play well with the Docker build process because there is no way to just download and build dependencies. [Zac Delventhal](https://stackoverflow.com/users/4284401/zac-delventhal) has already contributed the solution at [Stackoverflow](https://stackoverflow.com/questions/42130132/can-cargo-download-and-build-dependencies-without-also-building-the-application). Thank You! I make use of the work around with a dummy main program on first build in my Dockerfile.

## Getting started

Clone this repo and play around with docker.  

```
docker build -t mygreatapp .
docker run --init mygreatapp:latest
```
will build the docker image and during run output
```
[2019-03-19T18:09:52Z INFO  mygreatapp] An info Hello, world! mygreatapp someconfig-default-value
[2019-03-19T18:09:52Z WARN  mygreatapp] A warn Hello, world! mygreatapp someconfig-default-value
[2019-03-19T18:09:52Z ERROR mygreatapp] An error Hello, world! mygreatapp someconfig-default-value
```

Config the app with an environment variable 
```
docker run --init -e "MY_GREAT_CONFIG=hello" mygreatapp:latest
[2019-03-19T18:10:01Z INFO  mygreatapp] An info Hello, world! mygreatapp hello
[2019-03-19T18:10:01Z WARN  mygreatapp] A warn Hello, world! mygreatapp hello
[2019-03-19T18:10:01Z ERROR mygreatapp] An error Hello, world! mygreatapp hello
```

Configure the log level
```
docker run --init -e "MY_GREAT_CONFIG=hello" -e "RUST_LOG=trace" mygreatapp:latest
[2019-03-19T18:10:17Z TRACE mygreatapp] A trace Hello, world! mygreatapp hello
[2019-03-19T18:10:17Z DEBUG mygreatapp] A debug Hello, world! mygreatapp hello
[2019-03-19T18:10:17Z INFO  mygreatapp] An info Hello, world! mygreatapp hello
[2019-03-19T18:10:17Z WARN  mygreatapp] A warn Hello, world! mygreatapp hello
[2019-03-19T18:10:17Z ERROR mygreatapp] An error Hello, world! mygreatapp hello
```
And what about the image size. Well try
``` 
docker images
``` 

|REPOSITORY|TAG|IMAGE ID|CREATED|SIZE|
|----------|---|--------|-------|----|
|mygreatapp|latest|79bc9ddc7be4|2 hours ago|**3.88MB**|

## Use Docker in Your Rust project

1. Copy the `Dockerfile` from this repo to Your project
1. Set the BINARY_NAME_DEFAULT to the same as he name in Cargo.toml (watch out for dashes in the name, it will be replaced with _ in some cases ie. `rm deps/$BINARY_NAME*` will not work)
1. Set other config in ENV in the same way as MY_GREAT_CONFIG 
1. Replace the app name in the final line `CMD ["/mygreatapp"]` with the same as You assigned to BINARY_NAME_DEFAULT
