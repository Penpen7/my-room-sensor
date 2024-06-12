#!/bin/bash -e

docker build -t sensor:latest --target builder_arm_musl .

# run the container and get container id
container_id=$(docker run -d sensor:latest)

# get binary from container
docker cp $container_id:/app/target/aarch64-unknown-linux-musl/release/my-room-sensor ./my-room-sensor

# stop and remove container
docker stop $container_id
docker rm $container_id
