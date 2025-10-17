# ADR-030 Cache Manager Skeleton

- **Status:** Planning
- **Task:** TASK-030
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
With schema and flags defined, we need a central component to load/save cache files, manage backups, and expose APIs for downstream integrations (`research/refined-research-1/analysis/persistence_recommendations.md:76`).

## Decision
Implement a `CacheManager` responsible for:
- Discovering cache directories (`~/.codex/cache/...`).
- Loading JSON into strongly typed structures (`SchemaV1`).
- Writing atomic updates (write-to-temp + rename).
- Creating timestamped backups before destructive writes.
- Handling schema version mismatches (downgrade or reset workflow).

Manager lives in `codex-rs/core` alongside rollout infrastructure.

## Implementation Steps
1. Define cache directory layout constants.
2. Implement load/save methods with atomic filesystem writes.
3. Add backup/restore helpers.
4. Expose telemetry hooks (success/failure counters).
5. Unit tests for happy path and error handling.

## Success Criteria
- Load/save operations succeed for schema-compliant data.
- Errors surface user-friendly warnings.
- Backups created & restorable for corrupted writes.
- Code documented and linked from TRD.

## Related Research
- `research/refined-research-1/analysis/memory_models.md:3`
