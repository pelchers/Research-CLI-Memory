# Bootstrapping a New `.codex/` Directory

When starting a new repository that should follow the same conventions, copy the template directory:

1. Duplicate the root-level `.codex_template/` folder from this repo into the new project and rename it to `.codex/`.
2. Follow the checklist in `.codex/docs/project/how-to-bootstrap-codex-directory.md` inside the copied folder (the template includes a self-contained guide).
3. Replace placeholder docs (`overview.md`, `prd.md`, `trd.md`) with project-specific content. Leave `implementation-plan.md` marked “To be updated after research is completed” until the research phases deliver requirements.
4. Create fresh ADR folders/files per task; remove the template `adr/README.md` once populated.
5. Update `codex.md` if the new project has different workflows or branch strategies.
6. Maintain a research log (`research/refined-research-<n>/`) and keep `.codex/docs/project/research-implementation-plan.md` in sync with active phases.

This process ensures every project starts with the same Codex conventions established here.
