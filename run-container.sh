#!/bin/sh
docker run \
  -it \
  --name rust-test-container \
  --mount type=bind,source="$(pwd)"/apps,target=/apps \
  rust:slim-buster
