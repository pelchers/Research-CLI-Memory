# Phase 1 – Session Hook Integration Plan (Research)

## Objective
Describe how to integrate the short-term cache writer into Codex without altering behaviour until the feature flag enables persistence.

## Proposed Insertion Points
- `Session::run_task` (after `sess.record_conversation_items(&items_to_record_in_conversation_history)`) – capture the same payload used for history writes.
- `Session::send_event` – for event-only turns (e.g., approvals) ensure the cache also records the event metadata.
- Review-mode exit (`exit_review_mode`) – summarise isolated review conversation into a single cache entry.

## Hook Flow (Pseudocode)
```rust
if cache_config.enabled() {
    let cache_entry = ShortCacheEntry::from_turn(
        sub_id,
        &items_to_record_in_conversation_history,
        &turn_diff_tracker,
        turn_context.client.get_git_snapshot().await?,
    );
    cache_manager.write_short_cache(cache_entry).await?;
}
```

### `ShortCacheEntry::from_turn` Responsibilities
- Determine summary text (last assistant message or consolidated tool outputs).
- Collect actions:
  - Tool calls (`FunctionCall`, `LocalShellCall`, `CustomToolCall`) with status.
  - Approvals (approved/denied/abort).
  - Auto-compaction attempts (`token_limit_reached`).
- Attach follow-up notes derived from plan updates or explicit TODOs in the turn.

## Error Handling
- If `write_short_cache` fails, log warning and mark cache as “shadow-disabled” for the session (align with ADR-510).
- Continue execution normally; cache failures must not block main workflow.

## Testing Strategy Alignment
- Use fixtures generated from existing rollouts to simulate turn payloads.
- Validate that cache entries mirror history log contents (no duplicates).
- Check that disabling the feature flag bypasses cache writes entirely.

## Next Deliverables
- Map pseudocode to concrete helper functions in the prototype branch.
- Draft unit tests that consume mock `ShortCacheEntry` creation.
- Document configuration toggles for shadow mode vs full mode.
