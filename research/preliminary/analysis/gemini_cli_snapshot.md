# Gemini CLI (AI Studio) Memory Handling – Current Knowledge Gaps

- **Repository / Source Availability**
  - No official open-source repository discovered via GitHub search at time of scan (queries for `google gemini cli` returned third-party tools only).
  - Google AI Studio and Google Cloud documentation pages for the CLI return dynamic 404 shells when fetched without a browser environment, preventing text extraction.

- **Known Behaviours (from public usage reports)**
  - CLI ties sessions to AI Studio projects and can resume conversations via IDs.
  - Local config typically stored under `~/.config/google-ai-studio` (per community posts); includes credentials, cached threads, and last-selected project.
  - Memory persistence appears primarily server-side (threads stored in AI Studio workspace), with local cache for quick resume.

- **Open Questions**
  - Exact format of local cache (if any) and whether git metadata is captured alongside session logs.
  - Mechanisms for syncing long-term project goals versus per-session prompts remain unclear.
  - Whether the CLI exposes APIs for exporting/importing session memory or hooking custom storage backends.

- **Next Steps**
  - Access official docs through a headless browser or manual capture.
  - Inspect locally installed CLI (if available) to trace file system writes during tasks.
  - Reach out to community/release notes for confirmed behaviours regarding git state awareness.
