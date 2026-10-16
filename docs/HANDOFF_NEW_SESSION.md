# Telenetis — HANDOFF (new session)

- **Root**: `S:/rust/telenetis` (own git repo — extracted from the GSV kit tree on 2026-10-15; origin `github.com/platinoff/telenetis`, private). **Role:** portable **plugin** (Telegram shell). Hub is GSV `:9999`. Builds land in this crate's own `target/` (detached workspace; no GSV target sharing).
- **Owner 2026-09-17:** next `agi` from the GSV window **auto-picks this crate**. **Rabbit:** Redmi 9 on **wireless debug**. ADB on host `GSV/target/adb/platform-tools/adb.exe` — devices empty until owner sends pair `IP:port` + 6-digit code (`adb pair` / `adb connect`). Play from hub: Mini App `:9800` (live LAN `192.168.2.238`) + GSV `/api/edge`. Do not talk to poolAI `:8091` off-box. Do not grow this Mini App into a second brain. Runbook [`TWO_PHONES.md`](./TWO_PHONES.md) §KVM. Open: T23.1 ping, T26.1 pong after Start, T32.1 download watch.
- **What it is**: standalone Rust (Axum 0.8, Tokio) Telegram Mini App + Bot bridging the
  GSV Godfather channel — server on **port 9800**. Docs:
  [`README.md`](./README.md) · [`ops.md`](./ops.md)
  (Docker / systemd / Windows always-on supervisor).
- **Kit registration**: row in `S:/rust/GSV/docs/gsv/PRODUCTS.md`; keep-live band 224
  (Windows parity: watchdog copies debug → live and respawns `:9800`);
  GSV MCP probe `gsv_telenetis_health`.
- **Latest work (2026-10-15, first plugin band — sibling take-over)**: `:9800` now runs from
  this sibling tree (`target/live/telenetis.exe` via `target/debug/telenetis-live.exe`); the
  old nested `GSV/telenetis/target/` leftover is gone and the GSV watchdog resolves
  `telenetis_crate_dir` → `S:/rust/telenetis`. `/health` stops reporting the `0.1.0`
  placeholder: it now reads `env!("CARGO_PKG_VERSION")` and the crate bumped **0.1.0 → 0.2.0**
  (keep-live `telenetis.version` = `0.2.0`). Band journal starts here; GSV rows T1–T29 stay in
  the GSV log. Gate: fmt clean · clippy 0 · **330** green (314 lib + 4 live + 4 integration).
- **Latest work (2026-09-24, keep-green)**: fixed test rot in `proxy_get_stays_open_for_reads`
  (GET `/edge/upstream/poolai/...` asserted 404 but routing changed — a real upstream answers
  502 against a dead pool; the gate intent is "unknown service → 404", now uses `nosuch`).
  Toolchain: use `PATH="/c/Users/plati/.cargo/bin:..."` — MSYS2 `rustc 1.87` shadows the
  rustup 1.92.0-gnu pin otherwise (`encoding_rs`/`icu` need 1.88+). Gate: 330 green.
- **Latest work (2026-09-17, bands T1–T7, all ticket-driven)**: reopen-hint on stale initData (403),
  `answerWebAppQuery` builder (wiring verdict: no keyboard-flow → none needed),
  probe compatibility-mode, claim/next hints (snapshot + `/board`),
  Download auto-save (IDB is cache), sticky model auto-Use, worker chain memory
  (`payload.history`), WS tracker `GET /tracker` + client announce.
  Concept: [`CONCEPT.md`](./CONCEPT.md).
- **Tests**: `cargo fmt -- --check` → `cargo clippy --all-targets` (0) → `cargo test`
  (**314** lib + 4 live + 4 integration, green)
  (run in `S:/rust/telenetis`). Do **not** kill the live `:9800` copy before tests.
- **Health**: `GET http://127.0.0.1:9800/health` (GSV `keep_live.telenetis`, fresh = up).
- **NEXT pointer**: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).
