# How to Bootstrap `.codex/` in a New Repository

Use this checklist immediately after copying the template `.codex/` directory into a fresh project.

## 1. Initial Review
- Read `codex.md` to understand start-of-session protocol, ADR expectations, and git workflow.
- Review the placeholder project docs (`overview.md`, `prd.md`, `trd.md`, `implementation-plan.md`) so you know where to record project-specific information.

## 2. Customise Documentation
- Update `overview.md` with the project’s summary, scope, and non-goals.
- Fill in `prd.md` and `trd.md` with real requirements and technical decisions.
- Expand `implementation-plan.md` by adding phases, task IDs, dependencies, and references to future ADRs.

## 3. Prepare ADR Workspace
- Inside `.codex/adr/`, create a folder per planned task (e.g., `TASK-001`).
- For each task, draft a preliminary ADR using the format described in `codex.md` (Context, Decision, Implementation Steps, Success Criteria).

## 4. Align with Repository Structure
- Ensure the repo contains the directories referenced in `codex.md` (e.g., `research/` if you plan to store findings).
- If your workflow differs (different branch names, additional tooling), update `codex.md` accordingly.

## 5. Commit the Setup
- Run through the git policy in `codex.md` (commit on `master`, merge into active branch).
- Include references to the initial ADR IDs in the commit messages when you add them.
- Document any deviations from the template so future contributors understand the changes.

Following these steps gives you a clean starting point while preserving the conventions developed during the Codex Memory Research project.
