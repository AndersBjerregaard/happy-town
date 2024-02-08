#!/bin/bash
docker compose --env-file ../../config/secrets.env --file ./docker-compose.yaml up --build -d