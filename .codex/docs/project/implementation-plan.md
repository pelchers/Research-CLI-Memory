now# Implementation Plan – Codex Memory Persistence Upgrade

> **Legend**  
> Status: `Pending` (not started), `In Progress`, `Blocked`, `Complete`  
> ADR: See `.codex/adr/<task>/<file>.md`

## Phase 0 – Foundations
**Objective:** Establish core data contracts, guard rails, and supporting documentation.

| ID        | Status   | ADR Path                                                | Summary                                                      | Dependencies |
|-----------|----------|---------------------------------------------------------|--------------------------------------------------------------|--------------|
| TASK-010  | Pending  | `.codex/adr/TASK-010/ADR-010-cache-schema.md`           | Define schema versions, top-level keys, validation rules.    | —            |
| TASK-020  | Pending  | `.codex/adr/TASK-020/ADR-020-feature-flags.md`          | Introduce `memory.cache.enabled` flag + config defaults.     | TASK-010     |
| TASK-030  | Pending  | `.codex/adr/TASK-030/ADR-030-cache-manager.md`          | Skeleton cache manager (load/save, backups, migrations).     | TASK-010     |
| TASK-040  | Complete | `.codex/adr/TASK-040/ADR-040-codex-structure-readmes.md`| Document `.codex/` structure and template bootstrap process. | —            |

> **Note:** The deterministic implementation plan will be completed after research phases conclude. Current tasks remain `Pending` until research delivers final requirements.
