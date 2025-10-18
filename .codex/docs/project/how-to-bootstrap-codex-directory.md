# Bootstrapping a New `.codex/` Directory

When starting a new repository that should follow the same conventions, copy the template directory:

1. Duplicate the root-level `.codex_template/` folder from this repo into the new project and rename it to `.codex/`.
2. Follow the checklist in `.codex/docs/project/how-to-bootstrap-codex-directory.md` inside the copied folder (the template includes a self-contained guide).
3. Replace placeholder docs (`overview.md`, `prd.md`, `trd.md`, `implementation-plan.md`) with project-specific content.
4. Create fresh ADR folders/files per task; remove the template `adr/README.md` once populated.
5. Update `codex.md` if the new project has different workflows or branch strategies.

This process ensures every project starts with the same Codex conventions established here.
