# Phase 2 – Task Graph & Plan Integration (Research)

## Objectives
- Design the long-term project state (task graph) that complements the layered cache.
- Determine how Codex should synchronise `update_plan` events, ADR metadata, and git milestones.
- Identify edge cases (branch switching, task dependencies) and required tooling.

## Findings
1. **Graph Model**
   - Nodes represent tasks/subtasks mirrored from ADR IDs (e.g., `TASK-110`).
   - Edges encode dependencies; must remain acyclic to avoid deadlocks.
   - Nodes store: status, summary, ADR path, evidence (commits, analysis docs), associated cache entries.
2. **Storage**
   - Use `project_state.json` (long cache) as source of truth; mirror data into the sandbox or Codex UI for display.
   - Provide CLI helpers (`codex tasks show`, `codex tasks update`) that edit the graph via the cache manager.
3. **Plan Synchronisation**
   - Extend `update_plan` handler to:
     - Create nodes when they do not exist.
     - Update status when plan steps move to `in_progress`/`completed`.
     - Attach follow-up notes as `evidence`.
4. **Branch Awareness**
   - When switching branches, fork the task graph or prompt reconciliation (see Phase 3 research).
   - Graph entries must store branch info to detect if a task was completed on a different branch.
5. **Tooling**
   - Provide `codex tasks export` to produce Markdown/CSV summarising graph state (useful for research logs or PRDs).
   - Add validation to ensure dependencies reference valid nodes (leveraging sandbox validation work).

## Open Questions
- How should graph entries map to multi-phase research (R1 vs R2 vs R3)? Possible solution: use `milestone` field to differentiate.
- Should certain nodes collapse when merged upstream (e.g., research vs implementation tasks)? Needs usability testing during implementation.

## References
- ADR-010, ADR-030 (schema + cache manager).
- `.codex/docs/project/implementation-plan.md` (phase table mapped to graph nodes).
