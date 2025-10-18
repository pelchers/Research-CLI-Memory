# ADR-041 Research Status Tracking

- **Status:** Complete
- **Task:** TASK-041
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-18

## Context
Research and implementation planning required clearer status tracking and documentation across `.codex/` and `research/`. Previously, the implementation plan lacked status indicators, and there was no dedicated document summarising research phases and artefact locations.

## Decision
- Introduce icon-based status conventions (⬜/🔄/✅/⚠️) in `.codex/docs/project/implementation-plan.md`.
- Create `.codex/docs/project/research-implementation-plan.md` to catalogue research phases, their deliverables, and future plans.
- Update README files (`research/README.md`, `research/refined-research-1/README.md`) to reference the new research plan and structure.
- Mirror the same structure in `.codex_template/` so new repositories adopt the conventions.

## Consequences
- Implementation plan remains a placeholder until research completes but now records completed groundwork tasks (TASK-040, TASK-041).
- Research progress is auditable via the new research plan document and README cross-links.
- Templates guide future projects to adopt the same structure from the outset.

## Related Documents
- `.codex/docs/project/implementation-plan.md`
- `.codex/docs/project/research-implementation-plan.md`
- `.codex/docs/project/how-to-bootstrap-codex-directory.md`
- `research/README.md`
- `research/refined-research-1/README.md`
