# Technical Requirements – Codex Memory Persistence Upgrade

## Architecture Overview
- **Layered Cache**
  - Long-term: project_state.json (task graph, milestones, git snapshots, notes).
  - Short-term: sessions/<timestamp>_<branch>.json (per-turn summaries, approvals, follow-ups).
- **Core Services**
  - CacheManager: discovery, load/save, backups, retention policies.
  - TaskGraphService: CRUD APIs surfaced via CLI (codex tasks ...).
  - DriftDetector: compares cached git info vs live repo before each turn.
  - UX Layer: TUI banners + CLI commands showing cache status (Phase 4 research).

## Data Model Highlights
- Strongly typed via serde structs (see nalysis/phase1_cache_schema_structs.md).
- Task nodes map to ADR IDs and store status, dependencies, evidence.
- Short cache entries log 	urn_id, timestamp, summary, actions, follow-ups.
- Schema versioning + validation ensures safe migrations (Phase 1 sandbox covers duplicate/dependency checks).

## Integration Points
1. Session::run_task – capture per-turn payloads after ecord_conversation_items.
2. update_plan handler – sync plan updates into task graph nodes.
3. Reconciliation module – drift detection before each user turn; exposes CLI flows.
4. CLI/TUI – codex cache show, codex tasks ..., drift banners.

## Error Handling & Telemetry
- All writes asynchronous; failures warn and disable cache for session (shadow mode).
- Backups created before overwriting files; rollback command planned.
- Telemetry events: cache hit/miss, validation failure, drift type, reconciliation choice.

## Testing Strategy
- Schema validation sandbox (sandbox/) covers serde parsing + duplicate/dependency checks.
- Integration tests will script branch switches, rebases, detached HEAD scenarios.
- Performance benchmarks ensure <20 ms overhead per turn.

## Deployment & Rollout
- Feature flag defaults to disabled (memory.cache.enabled = false).
- Migration script seeds cache from historical rollouts (Phase 5 research).
- Documentation updates (PRD/TRD/README) accompany rollout; template bootstrap guide kept in sync.

## References
- Research docs: nalysis/phase1_*, phase2_*, phase3_*, phase4_*, phase5_*.
- ADRs TASK-010..530 align with implementation plan.
