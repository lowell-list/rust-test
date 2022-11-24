#!/bin/sh
docker run \
  -it \
  --rm \
  --name rust-test-container \
  --mount type=bind,source="$(pwd)"/app,target=/app \
  rust:slim-buster
