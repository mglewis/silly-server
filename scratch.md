Hit silly endpoint:

```
for i in {1..500}; do curl localhost:3030/silly > /dev/null 2>&1; done
```

Start up monitoring

```
docker-compose up
```

Query in prometheus the number of requests with

```
rate(silly_requests[1m])
```