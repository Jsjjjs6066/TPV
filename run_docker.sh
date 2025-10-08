sudo docker run -d --name tprl tprl:debian sleep infinity
sudo docker exec -it tprl /app/TPRL ./index.json
