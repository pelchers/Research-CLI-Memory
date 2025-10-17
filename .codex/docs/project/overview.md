# Memory Persistence Enhancement – Overview

## Goal
Deliver a durable project-memory layer for the Codex CLI so long-lived repositories retain task state, milestones, and git awareness across sessions and branch changes.

## Context
- Codex presently records raw rollouts and history logs, but lacks orchestration artefacts (roadmap, backlog, git drift detection).
- Prior research (see `research/refined-research-1/analysis`) identified layered cache + task graph as high-leverage improvements.
- The enhanced system must integrate smoothly with upstream Codex while remaining portable to a forked CLI if deeper experimentation is required.

## Scope
- Implement a layered cache (`project_state.json`, `sessions/<timestamp>_<branch>.json`).
- Introduce a persisted task graph aligned with the plan tool.
- Surface git drift alerts and reconciliation workflows.
- Provide documentation and CLI affordances to inspect, edit, and reset stored state.

## Non-Goals
- Replacing Codex execution engine or plan tool.
- Implementing autonomous multi-agent workflows (kept for future exploration).
- Shipping remote/cloud storage for cache data in this phase.
