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

Push stats to the push gateway

```
from prometheus_client import CollectorRegistry, Gauge, push_to_gateway

registry = CollectorRegistry()
g = Gauge('job_last_success_unixtime', 'Last time a batch job successfully finished', registry=registry)
g.set_to_current_time()
push_to_gateway('localhost:9091', job='batchA', registry=registry)
```

And then some different stats

```
from prometheus_client import CollectorRegistry, Gauge, push_to_gateway

registry = CollectorRegistry()
g = Gauge('job_last_success_unixtime', 'Last time a batch job successfully finished', registry=registry)
g.set_to_current_time()
push_to_gateway('localhost:9091', job='batchB', registry=registry)
```