#!/usr/bin/env bash
set -euo pipefail

IMAGE_NAME=${IMAGE_NAME:-caffalaughrey/gaelspell}
IMAGE_TAG=${IMAGE_TAG:-latest}

docker build -t "$IMAGE_NAME:$IMAGE_TAG" -f Dockerfile .

CID=$(docker run -d -p 5051:5000 "$IMAGE_NAME:$IMAGE_TAG")
trap 'docker rm -f $CID >/dev/null 2>&1 || true' EXIT

echo "Waiting for server..."
for i in {1..30}; do
  if curl -fsS http://localhost:5051/health >/dev/null; then
    break
  fi
  sleep 1
done

echo "Health check..."
curl -fsS http://localhost:5051/health >/dev/null

echo "API check..."
curl -fsS -X POST http://localhost:5051/api/gaelspell/1.0 \
  -H 'Content-Type: application/json' \
  --data '{"teacs": "Ba mhath liom abcdefxyz"}' | tee /dev/stderr | jq . >/dev/null

echo "OK"




