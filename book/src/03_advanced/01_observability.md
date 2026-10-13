# Observability

In WebAssembly serverless applications, more responsibility is shifted to the host runtime. This makes host-level observability even more essential for understanding application performance and debugging issues. After all, the runtime might be the bottleneck!

Observability is a host concern, it comes from the runtime, not your component. `spin` provides built-in support for OpenTelemetry (OTEL), an industry-standard observability framework that collects metrics, traces, and logs from distributed systems. This integration requires [Docker](https://docs.docker.com/desktop/) or Podman to run the OTEL collector and visualization tools locally.

To enable observability, first install the OTEL plugin which provides integration with OpenTelemetry tooling:

```sh
spin plugins update
spin plugin install otel
```

Then set up the OTEL stack. This downloads the necessary Docker containers, including collectors, storage backends, and visualization tools like [Jaeger](https://www.jaegertracing.io) for distributed tracing and [Grafana](https://grafana.com) for metrics dashboards:

```sh
spin otel setup
```

To start your application with observability enabled, run:

```sh
spin otel up
```

This command launches both your Spin application and automatically captures telemetry data from the Spin runtime.

Once your instrumented application is running, you can access the web interfaces for data visualization and analysis. Jaeger provides distributed tracing views to track request flows across components, Grafana offers customizable dashboards for metrics visualization, and Prometheus serves as the metrics storage and query interface:

```sh
spin otel open {jaeger,grafana,prometheus}
```
