#!/bin/bash

# Build and push the Node Dockerfile for x64
docker buildx create --use
docker buildx build --platform linux/amd64 -t andreivcodes/smscan-node:amd64 -f Dockerfile.node . --push

# Build and push the Node Dockerfile for ARM64 (Apple Silicon)
docker buildx create --use
docker buildx build --platform linux/arm64 -t andreivcodes/smscan-node:arm64 -f Dockerfile.node . --push

# Build and push the Smscan App Dockerfile for x64
docker buildx create --use
docker buildx build --platform linux/amd64 -t andreivcodes/smscan-app:amd64 -f Dockerfile.smscan --build-arg BUILDARCH=x86_64 . --push

# Build and push the Smscan App Dockerfile for ARM64 (Apple Silicon)
docker buildx create --use
docker buildx build --platform linux/arm64 -t andreivcodes/smscan-app:arm64 -f Dockerfile.smscan --build-arg BUILDARCH=arm64 . --push
