# ADR-330 Drift Scenario Test Suite

- **Status:** Planning
- **Task:** TASK-330
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Drift handling is error-prone; we need automated regression coverage for branch switches, rebases, detached HEAD, and dirty states (`research/refined-research-1/analysis/codex_gap_analysis.md:39`).

## Decision
Build an integration test suite that:
- Spins up temporary git repos with scripted histories.
- Exercises the comparator + reconciliation flows.
- Asserts cache outcomes (forked files, warning banners, audit logs).

Suite should run in CI with reasonable runtime (<2 minutes).

## Implementation Steps
1. Set up test harness using temp directories and git CLI.
2. Write cases for: branch checkout, rebase (history rewrite), uncommitted changes.
3. Validate reconciliation commands (fork, rebase, ignore).
4. Integrate suite into existing CI workflows.

## Success Criteria
- All drift scenarios covered with deterministic results.
- Failures provide actionable diagnostics.
- CI pipeline passes with suite enabled.

## Related Research
- `research/refined-research-1/analysis/codex_gap_analysis.md:39`
