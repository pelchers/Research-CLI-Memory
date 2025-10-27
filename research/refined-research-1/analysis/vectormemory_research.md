# Vector Memory & Journal Research

## Goals
- Detail how semantic retrieval (vector memory) and notebook-style journals complement the layered cache.
- Clarify storage options and integration points.

## Findings
1. **Vector Memory**
   - Embed code/document snippets + session summaries keyed by commit SHA + file path.
   - Storage options: SQLite + `pgvector`, local chroma, or file-based store.
   - Retrieval: query by current diff context to surface relevant history (ADR references, previous fixes).
   - Must respect feature flag; treat as optional module.
2. **Journals**
   - Append Markdown entries per session under `docs/ai-journal/YYYY/MM/DD.md`.
   - Link to cache entries and ADRs for auditing.
   - Provide CLI helper `codex journal append "..."`.
3. **Integration**
   - Vector embeddings can use sandbox data (fixtures + task graph) as seeds.
   - Journals feed evidence fields in task graph (makes plan updates human-readable).

## Open Questions
- Storage footprint vs. benefit; need heuristics for pruning embeddings.
- Should journals be versioned separately (e.g., Git submodule) for easier sharing?

## References
- `analysis/persistence_recommendations.md` (Vector Memory, Notebook sections).
