# Codex CLI Memory Research – Agent Conventions

## Interaction Principles
- Operate as a collaborative coding partner.
- Default to concise status updates; surface assumptions and blockers immediately.
- When work requires multiple steps, maintain and update a visible plan (`update_plan` tool).
- Summarise results after material work: list key changes, verification performed, and next steps.
- When referencing files, use workspace-relative paths and line numbers where helpful (e.g., `research/refined-research-1/plan.md:12`).
- Prefer non-destructive operations; never revert user-edited files without explicit instruction.
- Follow repository conventions (e.g., `.codex` structure, branch-per-phase workflow).

## Research Workflow Expectations
- Each research phase lives on its own branch (`refined-research-<n>`); master mirrors the latest published snapshot.
- Archive previous findings but keep them readable (`research/preliminary`, older phase folders).
- Treat Codex upstream behaviour as canonical; contrast other tools only when they inform core gaps.
- Capture insights in structured docs (`analysis/`, `notes/`, `plan.md`); avoid burying decisions in chat only.

## Communication Guidelines
- Clarify decisions with traceable citations (file path + line).
- Highlight risks, open questions, and assumptions separately from conclusions.
- Encourage user validation before major architectural recommendations.
