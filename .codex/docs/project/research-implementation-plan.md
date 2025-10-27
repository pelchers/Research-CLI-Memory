# Research Implementation Plan

This document tracks the research phases leading up to the deterministic implementation plan.

## Phase R0 – Preliminary Exploration (Status: ? Complete)
- Scope: Multi-provider comparison (Codex, Claude Code, Gemini).
- Artifacts: esearch/preliminary/analysis/*.md, esearch/preliminary/plan.md.
- Outcome: Identified Codex as the most revealing target for persistent memory enhancements.

## Phase R1 – Refined Codex Research (Status: ? Complete)
- Scope: Deep dive into Codex session persistence gaps and remediation strategies.
- Folder: esearch/refined-research-1/.
- Key deliverables:
  - nalysis/codex_current_state.md nalysis/codex_gap_analysis.md nalysis/persistence_recommendations.md nalysis/phase1_* docs.
  - 
otes/working_notes.md and sandbox summaries.
- Focus areas covered: short-term cache, schema, validation sandbox, hook/manager blueprints.

## Phase R2 – Long-Term State Research (Status: ?? In Progress)
- Scope: Task graph modelling, plan integration, manual edit strategies.
- Deliverables: nalysis/phase2_task_graph_research.md.

## Phase R3 – Drift & Reconciliation Research (Status: ?? In Progress)
- Scope: Git divergence handling, reconciliation UX/testing.
- Deliverables: nalysis/phase3_drift_research.md.

## Phase R4 – UX & Documentation Research (Status: ?? In Progress)
- Scope: TUI/CLI messaging, documentation set, telemetry.
- Deliverables: nalysis/phase4_ux_research.md.

## Phase R5 – Release & Upstream Planning (Status: ?? In Progress)
- Scope: Feature flags, migration tooling, upstream PR checklist.
- Deliverables: nalysis/phase5_release_research.md.

## Supporting Research Streams
- Agent orchestration: nalysis/agent_orchestration_research.md.
- Vector memory & journals: nalysis/vectormemory_research.md.

## Source Index
- Codex repo snapshot: esearch/refined-research-1/sources/codex/.
- Research notes: esearch/refined-research-1/notes/working_notes.md (links to sandbox results).
- Implementation plan & ADR references: .codex/ directory.

