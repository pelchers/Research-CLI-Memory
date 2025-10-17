# ADR-210 Task Graph Model

- **Status:** Planning
- **Task:** TASK-210
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
The layered cache hinges on a durable representation of goals, milestones, and subtasks so Codex can resume with actionable “what’s next” cues (`research/refined-research-1/analysis/codex_gap_analysis.md:10`).

## Decision
Model a task graph with:
- Nodes representing tasks/subtasks (id, title, status, owners, evidence).
- Directed edges for dependencies.
- Metadata for created/updated timestamps, related ADRs, linked files.
- Serialization format aligned with schema (TaskGraph → JSON).

Graph logic should support queries like “active tasks,” “blocked tasks,” and “next steps.”

## Implementation Steps
1. Define domain types (TaskNode, DependencyEdge, Status enum).
2. Integrate with schema (long-term cache).
3. Provide helper APIs (add/update nodes, traverse dependencies).
4. Write unit tests for graph operations.

## Success Criteria
- Graph can express hierarchical tasks used in implementation plan.
- API handles dependency cycles gracefully (validation error).
- Data persisted/read without loss.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:35`
