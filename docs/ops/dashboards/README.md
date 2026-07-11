# SessionLedger dashboards

`sessionledger-red.json` is an importable Grafana dashboard for the aggregate
HTTP RED counters exported by `sl-daemon` at `GET /metrics`.

## Prometheus scrape

Run the daemon on port 8080 and add this target to `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: sl-daemon
    scrape_interval: 15s
    static_configs:
      - targets: ["host.docker.internal:8080"]
```

Use `127.0.0.1:8080` instead when Prometheus runs directly on the host. On
Linux with Prometheus in a container, use the daemon's reachable host address
or add `host-gateway` for `host.docker.internal`.

## Import

In Grafana, choose **Dashboards → New → Import**, upload
`sessionledger-red.json`, and select the Prometheus data source. The dashboard
defaults to the `sl-daemon` scrape job and supports filtering by instance.

The Wave-6 exporter has no route labels or histogram buckets. The dashboard
therefore shows service-wide error ratio and mean duration; it intentionally
does not claim route-level or p95/p99 latency.
