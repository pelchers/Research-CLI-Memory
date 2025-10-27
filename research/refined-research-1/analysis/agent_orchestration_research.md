# Agent-Oriented Orchestration Research

## Purpose
Explore the alternative approach proposed in `persistence_recommendations.md` where specialised agents manage different memory scopes (planner, historian, executor).

## Summary
- **Architecture**: Supervisor agent routes user intent, invokes sub-agents with their own state stores (could interact with cache manager via APIs).
- **Use Cases**:
  - Planner maintains roadmap/task breakdown independent of live session.
  - Historian curates summaries + diffs for the long cache.
  - Executor focuses on short cache + validation events.
- **Advantages**: Enables autonomous progress tracking, proactive suggestions, and separation of concerns.
- **Challenges**: Coordination overhead, deterministic replay, potential latency.

## Research Notes
1. Agents should read/write the same cache schema to avoid divergence.
2. Need an orchestration policy (e.g., serialized agent turns, conflict resolution).
3. Slash command or config flag could enable “agent mode” as an optional feature.

## Next Steps
- Prototype minimal agent flow once baseline cache implementation lands.
- Measure impact on latency and error handling.

## References
- `.codex/docs/project/implementation-plan.md` (agent tasks appear in later phases).
- `analysis/persistence_recommendations.md` (Agent-Oriented Orchestration section).
