# Pre-Refinement Notes (Iteration 1)

## Context Recap
- Codex already persists each session as JSONL rollouts with `SessionMeta` and embedded `GitInfo`, plus a global `history.jsonl`, yet nothing composes those artefacts into a durable project roadmap. As a result, context resets across sessions and branch changes (`analysis/codex_current_state.md`, `sources/codex/codex-rs/core/src/rollout/recorder.rs`, `.../core/src/message_history.rs`, `.../core/src/git_info.rs`).
- The latest comparative scan showed that Codex exposes the richest persistence internals, while Claude Code and Gemini CLI provide limited or gated visibility. Findings on those tools remain archived in `../preliminary`.
- Candidate solutions already outlined include the layered cache architecture, agent/sub-agent orchestration, git-anchored vector memory, task graph datastore, and session journals (`analysis/memory_models.md`, `analysis/persistence_recommendations.md`).

## Scope Adjustment
- Further research should focus primarily—though not exclusively—on Codex resources, since they offer actionable implementation touch points (source code, docs, SDKs).
- Insights about Claude Code and Gemini CLI are treated as background context; revisit only if Codex exploration reveals gaps that require cross-tool validation.

## Deep-Dive: Codex Persistence Gap
- Rollout files faithfully capture per-turn events, metadata, and git snapshots, but they remain inert archives without higher-level synthesis. There is no mechanism to:
  - Track project milestones or long-term objectives across sessions.
  - Map session outputs to evolving git history (e.g., linking follow-up work to rebased commits).
  - Surface “what’s next” tasks when resuming after a break.
- This gap explains repeated user friction: re-explaining context, losing partially completed subtasks, and risking misalignment after branch divergence.
- The layered cache proposal directly targets these issues by introducing `project_state.json` for long-term orchestration and `sessions/<timestamp>_<branch>.json` for short-term continuity, backed by conflict resolution and summarisation pipelines (`analysis/memory_models.md`, `analysis/persistence_recommendations.md`).

## Next Actions (carried forward)
1. Reconfirm Codex persistence pipeline behaviour to ensure the cache design can hook into existing services (`plan.md`, Task 1).
2. Prototype the layered cache schema and define merge/reconciliation UX around git events (`plan.md`, Tasks 2–4).
3. Evaluate agent and vector-memory augmentations as optional modules, prioritising Codex upstream compatibility before exploring fork-specific enhancements.
4. Document any remaining questions that require revisiting `../preliminary` materials, so non-Codex context stays compartmentalised.
