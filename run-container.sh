#!/bin/sh
docker run \
  -it \
  -p 8080:8080 \
  --name rust-test-container \
  --mount type=bind,source="$(pwd)"/apps,target=/apps \
  rest-test-image
