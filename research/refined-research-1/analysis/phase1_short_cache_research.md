# Phase 1 – Short-Term Cache Research Notes

## 1. Lifecycle Hooks
- `Session::run_task` is the central loop where assistant responses, tool calls, and approvals are processed (`sources/codex/codex-rs/core/src/codex.rs:1648` onward).
- After every turn, the code builds `items_to_record_in_conversation_history` and calls `sess.record_conversation_items(...)` for non-review sessions (`.../core/src/codex.rs:1788`).
- The rollout recorder is invoked through `sess.persist_rollout_items` inside `Session::send_event` and other helpers; leveraging the same data ensures parity between rollout, history, and the short cache.
- **Recommended hook:** add a post-processing step immediately after `record_conversation_items` where the same payload is summarised for the short cache. This ensures the cache mirrors exactly what the long-term transcript receives without duplicating parsing logic.
- Review mode (`turn_context.is_review_mode == true`) uses an isolated in-memory history; short cache entries should reflect the final assistant message when exiting review, triggered via `exit_review_mode` (`.../core/src/codex.rs:1907`).

## 2. Data to Capture Per Turn
- Assistant messages (text + role) and reasoning blocks.
- Tool call inputs/outputs (`FunctionCall`, `FunctionCallOutput`, `CustomToolCall`, `LocalShellCall`).
- MCP invocations and results (converted via `convert_call_tool_result_to_function_call_output_payload`).
- Summaries of approvals and auto-compaction attempts (e.g., whether token limit triggered `compact::run_inline_auto_compact_task`).
- Git snapshot from `collect_git_info` should be taken *after* the turn completes to reflect post-turn state; reuse the helper already called during rollout creation (`.../core/src/rollout/recorder.rs:354`).

## 3. Concurrency & Performance
- Short cache writes must not block the task loop. Use `tokio::spawn` with a cloned `CacheManager` handle to perform I/O asynchronously, mirroring existing history writes (`Op::AddToHistory` spawns a task for `message_history::append_entry`, `.../core/src/codex.rs:1396`).
- Atomic file writes: adopt the `tempfile` + rename approach inside `CacheManager` to avoid partial writes if the process crashes mid-operation (see ADR-030).
- Throttle cache pruning using the existing retention policy (ADR-120) to avoid repeated summarisation for back-to-back turns.

## 4. Error Handling
- Cache writes should never panic; log warnings and continue execution if persistence fails. Consider downgrading to “shadow mode” for the session and surfacing a TUI banner (ADR-510).
- When git info cannot be collected (e.g., repository unavailable), annotate the cache entry with `git: null` instead of failing the write.

## 5. Testing Strategy
- **Unit tests:** mock `CacheManager` to ensure the turn hook receives the exact payload as `record_conversation_items`. Validate that review-mode turns are skipped or summarised correctly.
- **Integration tests:** run scripted conversations (assistant message, tool call, approval) and assert that `codex cache show --recent` reflects the sequence.
- **Regression tests for token-limit flow:** ensure auto-compaction events are recorded when `token_limit_reached` triggers (`.../core/src/codex.rs:1827`).
- **Performance benchmarks:** measure added latency per turn. Target <20 ms overhead as noted in ADR-110. Use the existing OTEL `user_prompt` + `notify` instrumentation to bracket measurements.

## 6. Telemetry & Logging
- Increment counters when cache writes succeed or fail (ADR-430). The hook is a natural point to emit metrics via `turn_context.client.get_otel_event_manager()`.
- Log reconciliation state when git drift is detected on subsequent turns (link to Phase 3 work).

## 7. Open Questions
- How to handle multi-part assistant messages (streaming deltas) before the final `TaskComplete` event? Proposal: accumulate deltas in-memory and write to cache only when the final turn item is flushed.
- Should we redact sensitive tool outputs (e.g., large diffs) in short cache summaries? Need policy before implementation.
- Review mode currently bypasses `record_conversation_items`; do we want to persist review transcripts in the cache for auditing? If yes, convert `review_thread_history` into a summary when `exit_review_mode` fires.

## References
- Session run loop: `research/refined-research-1/sources/codex/codex-rs/core/src/codex.rs:1648-1940`.
- Rollout recorder meta capture: `.../core/src/rollout/recorder.rs:304-366`.
- Existing history persistence: `.../core/src/codex.rs:1388-1398`.
- ADR cross-links: `.codex/adr/TASK-010/ADR-010-cache-schema.md`, `.../TASK-110/ADR-110-turn-hook.md`, `.../TASK-120/ADR-120-short-cache.md`, `.../TASK-130/ADR-130-cli-short-cache.md`.
