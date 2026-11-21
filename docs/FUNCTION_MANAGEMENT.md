# telenetis — FUNCTION_MANAGEMENT (симбіоз)

**Companion:** `S:/rust/GSV/docs/gsv/GSV_FUNCTION_MANAGEMENT.md` · [`GSV_ECOSYSTEM_SYMBIOSIS.md`](../../GSV/docs/gsv/GSV_ECOSYSTEM_SYMBIOSIS.md) · [`GSV_AGI_PATH.md`](../../GSV/docs/gsv/GSV_AGI_PATH.md)  
**Шар:** plugin (commutation/UI) · **Repo:** `https://github.com/platinoff/telenetis` · **Branch:** `main`

## 1. Оголошені функції

| Функція | Модулі/шляхи | Статус | Примітка |
|---------|--------------|--------|----------|
| Telegram bot + Mini App | `src/`, `ui/`/docs as present | active | Commutation/UI plugin for GSV ecosystem |
| initData HMAC / Telegram auth checks | Telegram/webhook/auth modules | active | Secret-handling rules apply; never echo tokens |
| Snapshot/UI/ticket actions integration | GSV ticket/telemetry integration surfaces | active | GSV remains bus/state owner; telenetis = commutation |
| Ops/docs (two-phones, lab, ops) | `docs/ops.md`, `docs/TWO_PHONES.md`, `docs/LAB.md` | active docs | Operational guidance; owner-gated secrets/rotation |

## 2. Звірка / gates

- Rust gates: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (targeted/full as appropriate for box).
- GSV keep-live/telenetis status checked via GSV MCP/ops; token/webhook operations are owner-gated.

## 3. Дублі / межі (посилання на master §3)

- **D5** Telegram: distributed pair — GSV owns bus/state; telenetis owns commutation/UI. No third Telegram owner.
- **D7** Rolling logging: intentional per-repo.

## 4. Owner-рішення / наступні кроки

- Owner-gated: token rotate, reply ownership, webhook↔poller split decisions.
- Keep folder-scoped MCP/wiring aligned with GSV kit; no secrets in git/docs.
- Update skeleton when commutation boundaries or ops model change.
