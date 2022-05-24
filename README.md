# Rust Blog

This code accompanies a blog post written [here](TODO: add link).

## Build

To make things easier, we suggest using a Docker image.

```
docker build . -t rustblog
```

Spin up a container using the newly build image

```
docker run --rm -it -v $(pwd):/code/ rustblog bash
```

Build and run the project.

```
make
LD_LIBRARY_PATH=cppcwrapper/ ./cpp/MyCppApp
```
