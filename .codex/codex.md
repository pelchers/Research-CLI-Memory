# Codex Memory Research – Agent Conventions

## Start-of-Session Protocol
On every new chat session:

1. **Scan the workspace**
   - `.codex/docs/project/` – PRD, TRD, implementation plan, overview.
   - `.codex/adr/` – preliminary ADRs for active subtasks.
   - `research/` – `refined-research-*` (current phase), `preliminary/` (archive).
2. **Identify the active phase**
   - The highest numbered `refined-research-*` folder is the current focus.
   - Master branch mirrors the last published snapshot; each phase has its own branch.
3. **Load working context**
   - Read `plan.md` inside the current `refined-research-*` folder.
   - Review relevant ADRs (`.codex/adr/TASK-*/`) before touching code or docs.
4. **Update plan tooling**
   - Maintain a live task plan via the `update_plan` tool whenever work spans multiple steps.

## Repository Structure & Hierarchy
```
workspace/
├─ .codex/                # Conventions, project docs, ADRs
├─ research/
│  ├─ refined-research-1/ # Active research phase
│  └─ preliminary/        # Archived findings
└─ .git/                  # Branch-per-phase workflow
```
- `.codex/` instructions override historical notes unless explicitly superseded.
- `research/` contains the evidence base; cite files (e.g., `research/refined-research-1/analysis/codex_gap_analysis.md:12`) when referencing findings.

## Planning & ADR Requirements
- **Every subtask must have a preliminary ADR** before implementation work begins.
  - Location: `.codex/adr/TASK-XYZ/ADR-XYZ-<slug>.md`
  - Status stays `Planning` until implementation completes.
  - Sections to include: Context, Decision, Implementation Steps, Success Criteria, Assets/Artifacts.
- ADRs serve as living blueprints; update them with outcomes once the subtask is delivered.
- Commits must reference the corresponding ADR ID in the message body.

## Execution Workflow
1. Review the implementation plan and relevant ADR.
2. Break the work into verifiable steps; call `update_plan`.
3. When editing files, read immediately before modifying.
4. Prefer additive, non-destructive changes; never delete user artefacts without approval.
5. Validate assumptions with citations from `research/` or project docs.
6. Summarise results with:
   - What changed (paths + line refs)
   - Tests/validation performed (if any)
   - Next steps or remaining risks

## Branching & Git Discipline
- Branch naming: `refined-research-<n>` for active research (mirrors plan phases).
- Master must stay in sync with the latest stable snapshot.
- Standard commit cadence for this project:
  1. Work from `master`.
  2. Commit changes (`git commit -m "…ADR-### …"`).
  3. `git push origin master`.
  4. `git checkout refined-research-<n> && git merge master && git push`.
- Never skip pushing master before updating the phase branch.

## Quality Gates
- Documentation updated (`.codex/docs/project`, ADRs, plan).
- References include file paths + line numbers.
- No unresolved TODOs left in ADRs or plan.
- All new conventions reflected in `.codex/codex.md`.

## Communication Guidelines
- Be explicit about the source of truth (cite files/lines).
- Flag risks, blockers, or unanswered questions separately from conclusions.
- If instructions conflict, prefer the newest `.codex/` guidance, then phase plan, then archived material.
- Keep summaries concise but complete—mirror the structure used in research responses (gaps, proposed fixes, next steps).
