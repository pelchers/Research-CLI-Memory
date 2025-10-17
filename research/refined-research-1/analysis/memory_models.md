# Memory Persistence Models (Draft)

## 1. Layered Cache Strategy
- **Scope**: Split long-term project scaffold from short-term session context.
- **Artifacts**:
  - `project_state.json` (long cache): project goals, milestones, dependency graph, git checkpoints, canonical artefact links.
  - `sessions/<timestamp>_<branch>.json` (short cache): turn summaries, open subtasks, pending approvals, scratchpad notes.
- **Lifecycle**:
  1. On launch: load long cache, merge the most relevant session file for current branch/head, hydrate prompt preamble.
  2. During turn: update in-memory short cache; record references to git diff, executed commands, approvals.
  3. On turn completion: flush short cache delta; when milestones complete, append to long cache and snapshot git commit hash.
  4. On branch change/rebase: CLI detects mismatch, prompts reconciliation (e.g., fork cache per branch or rebase cache entries).
- **Git Hooks**: Store commit hash + branch on every cache update; optionally refresh cache on `post-commit` or `post-merge`.
- **Open Questions**: Conflict handling for concurrent edits, summarisation budgets, cross-branch linking, partial rewinds.

## 2. Agent-Oriented Orchestration
- **Scope**: Assign sub-agents (planner, historian, executor) to maintain different memory bands, coordinated by a supervisor.
- **Artifacts**: Structured store (SQLite/JSON) with tables for tasks, transcripts, evidence; agent queue metadata for pending actions.
- **Lifecycle**:
  1. Supervisor interprets user intent, updates task graph.
  2. Specialist agent executes (e.g., historian updates knowledge base from rollouts, planner refreshes roadmap).
  3. Consolidation writes canonical summary back to shared store; supervisor emits human-readable recap.
- **Git Hooks**: Agents monitor git state changes, automatically branch caches, or request user approval before proceeding on stale branches.
- **Open Questions**: Arbitration when agents disagree, ensuring reproducibility, bounding latency, fallback when an agent fails.

## 3. Additional Candidates
- **Vector Memory with Git Anchors**: Encode repo artefacts and chat excerpts; embed commit metadata to retrieve relevant slices for prompt grounding.
- **Notebook-Style Session Logs**: Append structured Markdown per interaction; rely on git for diffing and timeline navigation; suits human audit trails.
- **Task Graph Datastore**: Persist DAG of objectives and dependencies; integrate with CLI commands to mark progress and surface blockers.
- **Hybrid Approaches**: Combine layered cache (state) + vector store (retrieval) + journal (human oversight) for redundancy and transparency.

## Evaluation Signals (to refine)
- Persistence robustness across branch rebases, merges, force pushes.
- Ease of user inspection, manual editing, and version control.
- Alignment with Codex CLI architecture for potential upstream contribution.
- Storage footprint and performance implications in large repos.
- Complexity vs. payoff for both open-source contribution and bespoke CLI.
