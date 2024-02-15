#!/bin/bash
source .env

docker build $(pwd) -t cars_schema_up
docker run \
    -w /code \
    --env-file .env \
    --network "$NETWORK" \
    cars_schema_up \
    npm run migration