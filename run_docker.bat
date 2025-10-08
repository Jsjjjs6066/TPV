docker run -d --name tprl tprl:debian sleep infinity
docker exec -it tprl /app/TPRL ./index.json
