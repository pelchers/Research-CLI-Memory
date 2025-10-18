# Architecture Decision Records

Each task in the implementation plan has a dedicated subdirectory under this folder (e.g., `TASK-110`). Inside each task folder, ADR files capture planning and execution details using the naming convention `ADR-###-short-name.md`.

## Usage
1. Before starting a subtask, create or update the corresponding ADR with Status = `Planning`.
2. During implementation, record decisions, trade-offs, and validation steps.
3. After completion, update Status to `Complete` and document outcomes.
4. Reference ADR IDs in commit messages (e.g., `ADR-120`) and in the implementation plan.

## Structure Requirements
- One task per subdirectory.
- ADRs must include Context, Decision, Implementation Steps, Success Criteria, References, and Status.
- Do not delete historical ADRs; append superseding records if requirements change.
