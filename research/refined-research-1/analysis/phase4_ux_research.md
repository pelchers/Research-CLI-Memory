# Phase 4 – UX & Documentation Research

## Objectives
- Determine how Codex should expose cache information (tasks, drift alerts, status) to users.
- Define documentation deliverables for public release.
- Align UX updates with existing Codex CLI/TUI capabilities.

## Findings
1. **TUI Enhancements**
   - Add status banner summarising:
     - Cache mode (disabled/shadow/active).
     - Active tasks + next steps (pulled from task graph).
     - Drift warnings (Phase 3).
   - Provide hotkeys/commands to open task graph view (`Ctrl+T` analog).
2. **CLI Commands**
   - `codex cache show` – summary of long/short cache entries.
   - `codex tasks show|complete|edit` – interact with task graph.
   - `codex cache reconcile` – handle drift actions.
3. **Documentation Set**
   - Update PRD/TRD when research concludes.
   - Produce user guide covering:
     - How the cache works.
     - How to inspect/edit state.
     - Troubleshooting drift/validation errors.
   - Publish migration guide (Phase 5) for existing Codex users.
4. **Telemetry & Notifications**
   - Telemetry events for cache hits/misses, validation failures.
   - Desktop notifications for drift or cache disablement (optional; config-driven).

## Open Questions
- Should TUI provide inline editing for tasks, or rely on CLI commands? Prototype to measure usability.
- How to present large task graphs succinctly? Potential solution: filter by milestone or ADR.

## References
- ADR-410/420/430 placeholders.
- `persistence_recommendations.md` (Layered cache + UX table).
