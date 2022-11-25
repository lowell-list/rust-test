#!/bin/sh
docker run \
  -it \
  --name rust-test-container \
  --mount type=bind,source="$(pwd)"/app,target=/app \
  rust:slim-buster
