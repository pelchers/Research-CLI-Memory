# ADR-430 Cache Telemetry & Metrics

- **Status:** Planning
- **Task:** TASK-430
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
To evaluate adoption and reliability we must instrument cache operations (writes, failures, drift events) and expose metrics for analysis (`research/refined-research-1/analysis/persistence_recommendations.md:99`).

## Decision
Add telemetry hooks emitting:
- Counters: cache writes, read failures, drift detections, reconciliation actions.
- Gauges: cache file sizes, turn latency impact.
- Logs: warning/error level with ADR/task references.

Metrics integrate with existing Codex telemetry pipeline (OTEL events).

## Implementation Steps
1. Define metric names + tags (task id, mode, outcome).
2. Emit counters/gauges from cache manager and reconciliation flows.
3. Document metrics in TRD and docs.
4. Add tests to ensure metrics recorded during simulations.

## Success Criteria
- Metrics visible in development telemetry backend.
- Zero additional panics/crashes due to instrumentation.
- Documentation explains how to interpret metrics.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:99`
