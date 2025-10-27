# Research Plan

## Immediate Objectives
- Deepen understanding of Codex CLI memory behaviour and highlight outstanding persistence gaps.
- Validate long/short cache layering as a viable contribution path for Codex.
- Scope complementary models (agent orchestration, task graph, vector retrieval) specifically against Codex architecture.

## Task Backlog (✅ Done · 🔄 In Progress · ⬜ Pending)
1. ✅ Reconfirm Codex session persistence pipeline (see `analysis/codex_current_state.md`, `analysis/codex_gap_analysis.md`).
2. ✅ Elaborate cache-tier architecture (see `analysis/phase1_short_cache_research.md`, `analysis/phase1_cache_schema_*`, `analysis/phase1_cache_manager_blueprint.md`).
3. ✅ Outline agent orchestration alternative (`analysis/agent_orchestration_research.md`).
4. ✅ Evaluate supporting strategies (vector memory/journals: `analysis/vectormemory_research.md`; task graph: `analysis/phase2_task_graph_research.md`).
5. ✅ Compare approaches for upstream contribution (`analysis/persistence_recommendations.md`, `analysis/phase5_release_research.md`).
6. 🔄 Maintain recommendation matrix with implementation risks, effort, migration considerations (`analysis/persistence_recommendations.md` will continue to evolve as prototypes land).

## Decision Log
- _In Progress_: Confirm evaluation metrics (persistence fidelity, UX friction, portability) — see draft matrix in `analysis/persistence_recommendations.md`.
- _Pending_: Decide on local-only vs. optional remote sync storage.
- _Pending_: Select prototype path if user requests implementation phase.

## Notes
- Scope remains Codex-first; insights from other CLIs live in `../preliminary` for reference only.
- Ensure every proposal captures git state handling (branches, commits, WIP) in the persistence story.
