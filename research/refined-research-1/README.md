# Refined Memory Persistence Workspace

This iteration narrows the investigation to OpenAI Codex CLI, the only source that exposed concrete session persistence internals. The structure mirrors the original research tree so earlier notes archived under `preliminary/` remain traceable.

- `plan.md` — current backlog and decision tracker scoped to Codex-focused work.
- `analysis/` — refined evaluations of persistence gaps and solution models referencing Codex internals (e.g., `codex_gap_analysis.md`, `phase1_short_cache_research.md`).
- `notes/` — active scratchpad for Codex-specific observations (`working_notes.md`).
- `sources/` — Codex repository snapshot and citations required for deeper analysis.
- `pre-ref-1.md` — summary of context from previous phases and next steps.

Use `.codex/docs/project/research-implementation-plan.md` to understand how this phase fits into the overall research roadmap.

All new deliverables for the refined phase should live under this directory while legacy findings stay in `../preliminary`.
