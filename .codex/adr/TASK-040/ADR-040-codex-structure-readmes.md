# ADR-040 Codex Directory Documentation

- **Status:** Complete
- **Task:** TASK-040
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-18

## Context
To onboard future contributors and enable rapid replication of this repo’s conventions, the `.codex/` directory required descriptive README files mirroring the structure found in the reference `.claude/` project. Prior to this ADR, several subdirectories (e.g., `agents/`, `ci/`, `toolchains/`, `workflows/`) lacked guidance, and the template folder did not explain how to initialise these areas.

## Decision
- Add README files to each top-level `.codex/` subdirectory describing its purpose, current usage, and expectations for the memory persistence project.
- Mirror the same documentation in `.codex_template/` so new repositories inherit the structure without project-specific artefacts.
- Introduce a bootstrap guide (`.codex/docs/project/how-to-bootstrap-codex-directory.md`) and reference it from the template root README.

## Implementation Summary
1. Documented `.codex/`, `adr/`, `docs/`, `agents/`, `ci/`, `toolchains/`, and `workflows/`.
2. Created equivalent README files in `.codex_template/` and added missing directories.
3. Recorded this ADR and updated the implementation plan (TASK-040).
4. Synced changes across `master`, `refined-research-1`, and the `codex-template` branch.

## Success Criteria
- Contributors can understand each `.codex/` subdirectory without external knowledge.
- Template users receive instructions for populating empty areas.
- Future ADRs reference the template when new conventions are introduced.

## References
- `.codex/README.md`
- `.codex/docs/project/how-to-bootstrap-codex-directory.md`
- `.codex_template/README.md`
