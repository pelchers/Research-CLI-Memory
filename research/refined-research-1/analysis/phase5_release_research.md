# Phase 5 – Release & Upstream Strategy Research

## Objectives
- Plan the opt-in rollout, migration tooling, and upstream contribution workflow.
- Ensure safety nets (flag gating, backups, migration scripts) are specified before implementation.

## Findings
1. **Feature Flag Rollout**
   - Default `memory.cache.enabled = false` with `write_mode = "shadow"`.
   - Telemetry monitors failure rate; upgrade to `active` once confidence established.
2. **Migration Script**
   - Command: `codex cache migrate --from-rollouts`.
   - Steps:
     1. Scan historical rollouts, build task graph + short cache seeds.
     2. Validate results (schema + dependency checks).
     3. Backup existing cache files before applying.
3. **Upstream Contribution**
   - Package includes:
     - Feature overview.
     - Test results (schema/unit/integration + sandbox logs).
     - Risk assessment & fallback plan.
     - Documentation updates (README, docs/).
   - Submit PR referencing ADR IDs and research docs.
4. **Release Checklist**
   - ✅ Research docs completed (this repo).
   - 🔄 Prototype merged behind flag.
   - 🔄 Beta testing with opt-in config.
   - ⬜ Default-on release once metrics stable.

## Open Questions
- Which versions of Codex CLI need support (back-compat)? Document in TRD when implementation starts.
- Should migration script be reversible? Likely yes (store backups + `codex cache rollback`).

## References
- ADR-510/520/530 placeholders.
- `.codex/docs/project/how-to-bootstrap-codex-directory.md` for onboarding new repos.
