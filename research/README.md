# Research Workspace Conventions

This directory tracks multi-phase investigations into Codex (and related CLI) memory persistence. Each phase produces its own self-contained workspace while preserving earlier findings for reference.

## Directory Layout
- `preliminary/` – archive of exploratory notes and cross-tool findings captured before the Codex-focused refinement.
- `refined-research-<index>/` – codex-centric research phases (`refined-research-1` is the current iteration). Future phases should increment `<index>` (e.g., `refined-research-2`).

## Required Structure for Refined Phases
Each `refined-research-<index>/` directory must contain:
- `README.md` – overview of the phase scope and how it differs from prior iterations.
- `plan.md` – phase-specific backlog and decision log.
- `analysis/` – structured write-ups (e.g., `codex_current_state.md`, solution models, recommendations).
- `notes/` – scratchpad-style working notes.
- `sources/` – material pulled from upstream repositories or documentation needed for the phase.
- `pre-ref-<index>.md` – bridge document summarising prior context, reasons for initiating the phase, and immediate next steps.

## Creating a New Phase
1. Archive the previous phase (no deletion) and create `refined-research-<next>` by cloning the structure above.
2. Copy forward only materials still relevant; leave broader context in earlier folders to reduce clutter.
3. Update `plan.md` and `README.md` to reflect new goals, assumptions, and scope boundaries.
4. Populate `pre-ref-<next>.md` with:
   - Recap of prior findings.
   - Justification for the new phase.
  - Key gaps or questions being targeted.
5. Record any deviations from these conventions in the new phase’s README.

## Referencing Older Material
- Treat `preliminary/` (and older `refined-research-*` directories) as read-only archives unless explicitly updating errata.
- When citing historical findings, reference the relevant file path (e.g., `preliminary/analysis/claude_code_snapshot.md`).

Following this structure keeps each phase auditable, reduces context loss, and makes it easy to spin up future iterations with minimal overhead.
