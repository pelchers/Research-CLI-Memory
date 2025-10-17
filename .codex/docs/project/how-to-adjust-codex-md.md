# How to Adapt `codex.md` for a New Repository

This guide outlines the steps to transform a reference AI convention file (e.g., from another project’s `.claude/CLAUDE.md`) into a Codex-specific `codex.md` that mirrors the conventions we established in this project. Use it when bootstrapping a fresh repository.

---

## 1. Collect Source Material
1. Identify the canonical convention file in the reference project (e.g., `.claude/CLAUDE.md`, `.claude/claude-verbose.md`).
2. Gather any supplemental docs that explain workflows (implementation plan, ADR policies, branch strategy).
3. Note any project-specific terminology or ASCII markers you may want to retain or replace.

---

## 2. Extract Core Sections
From the reference file, copy the following structural elements:
- **Start-of-session protocol** (how the agent bootstraps context).
- **Repository awareness** (what directories to read, precedence rules).
- **Planning & ADR requirements**.
- **Standard workflow / On every code change checklist**.
- **Branching and git policy**.
- **Quality gates**.
- **Communication guidelines**.

Keep the original level of specificity (e.g., enumerated steps, bullet lists) so conventions remain explicit.

---

## 3. Remove Project-Specific Assumptions
Replace references that assume a fixed repo structure:
- Swap hard-coded folder names (e.g., `refined-research-*`) with more flexible guidance:
  - “Inspect subdirectories to determine the latest phase; confirm via plan files and timestamps.”
- Avoid referencing files that do not exist in the new repo.
- Generalise instructions to rely on content discovery rather than strict naming.

Checklist:
- [ ] No absolute paths to foreign repositories.
- [ ] No assumptions about branch names unless they are part of your new workflow.
- [ ] References point only to files you will recreate (PRD, TRD, implementation plan, ADR folder).

---

## 4. Re-anchor Conventions to the New Repo
1. Update directory references to match the target repo (`.codex/`, `.codex/adr/`, `.codex/docs/project/`, etc.).
2. Ensure the implementation plan and ADR structure you want to follow already exist or will be created (see `implementation-plan.md`).
3. Explicitly instruct readers to rely on the latest `.codex/` guidance first, then plans and ADRs, then archived research.

---

## 5. Preserve Detailed Workflow Checklists
The “On Every Code Change” and other checklists are critical—carry them over verbatim unless they conflict with the new project’s process. Adjust only the parts that reference systems you are not using (e.g., remove references to automation agents that don’t exist in the new repo).

---

## 6. Review for Completeness
Before finalising:
- Verify each section is present and ordered logically.
- Confirm all steps reflect the new repo’s actual workflows (branch strategy, ADR storage, validation requirements).
- Proofread for accuracy and clarity.

Optional: Add a quick reference checklist at the end tailored to your project.

---

## 7. Version Control Hygiene
1. Commit the new `codex.md` with a message referencing the relevant planning ADR (e.g., `ADR-###`).
2. Push to `master`, then merge into the active phase branch (if applicable).
3. Document any changes to conventions in the implementation plan or ADRs.

---

## Example Adaptation Flow
1. Copy reference `.claude/claude-verbose.md` content.
2. Remove provider-specific context (e.g., relation to `.claude/`, `.gemini/`).
3. Replace repository structure with `.codex/`, `research/`, and ADR hierarchy.
4. Insert branch policy and planning rules as per your new workflow.
5. Double-check checklists include TDD, validation, ADR updates, commit requirements.
6. Use `codex_template.md` (same directory as this guide) as a starting point for the new repo—update only the sections that need project-specific wording.
7. Commit and push; update documentation to point future maintainers to this “How to” guide.

Following these steps ensures the `codex.md` in any new repo inherits the best practices developed here without dragging in stale or irrelevant references.
