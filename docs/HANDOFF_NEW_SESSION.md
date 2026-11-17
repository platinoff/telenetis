# Передача контексту новій сесії (Telenetis)

**Оновлено:** 2026-11-17 (Symbiosis P1 ✅ — цей файл створено; GSV канон [`docs/gsv/GSV_ECOSYSTEM_SYMBIOSIS.md`](../../GSV/docs/gsv/GSV_ECOSYSTEM_SYMBIOSIS.md))

> **Екосистемний канон (2026-11-17):** [`../../../GSV/docs/gsv/GSV_ECOSYSTEM_SYMBIOSIS.md`](../../GSV/docs/gsv/GSV_ECOSYSTEM_SYMBIOSIS.md)
> — GSV «тато» (операційний план: Galaxy UI, tickets board, keep-live, mesh, MCP) ·
> poolAI «мама» (FM/management план). Telenetis — **commutation** (switching) плагін:
> Telegram Mini App + workers/flows + GSV forward. Не переносимо й не розширюємо в галактичний
> оркестр — перемикання лише.

## Призначення

Telenetis — плагін, що цілить у **саму комунікацію** між:
- Telegram Bot API (webhook, Mini App, initData HMAC-SHA256, theme/platform/backoff/cold-start UX),
- GSV-хабом (`/api/tickets/*`, `/api/telegram/*`, `:9999`) — forward Mini-App дій робітників
  (claim/done/error/reclaim), presence та flow sync,
- workers (роздача task envelopes подій у подійну шину),
- Mini App на `:9800` (`/health`, `/api/verify`, `/api/board/*`, `/api/snapshot`).

Не входить: inference, tensor ops, KVM, APK edge, 49-device RAID — це engines (llama-rs / rusty-engine)
і Hub (GSV). Не SSU-оркестратор.

## Стан на 2026-11-17 (знято з живого боксу)

| Що | Значення |
|----|----------|
| HEAD | `7efad09` (master, synced origin/master) |
| Crate | **0.2.0** (`Cargo.toml`) |
| Port | **:9800** (live; `/health`) |
| Wiring | OpenCode/Grok/Cursor `ok` (GSV audit 2026-11-17) |
| Keep-live | GSV watchdog мульти-пробує `:9800` (`telenetis_alive`) + `telenetis_debug_newer` (band 228) |
| MCP peer | `gsv_telenetis_health` (band 226, fail-open при `down`) |
| Tests | `cargo fmt -- --check` → `cargo clippy --all-targets` → `cargo test` (~177 у band 223; точна кількість — на наступному дрені) |
| Збірка | MSYS2 bash; `unset RUSTUP_TOOLCHAIN`; windows-gnu ціль |

## Ключові файли

- `AGENTS.md` — локальний канон (те, що змінити не треба).
- `src/security/initdata.rs` — initData HMAC-SHA256 verify (band 214).
- `src/ui/miniapp.rs` + `src/ui/static/app.js` — Mini App layer (band 215).
- `src/stream/backoff.rs` + `src/stream/ws.rs` — live stream primacy (band 216).
- `src/actions.rs` — BoardAction (claim/done/error/reclaim) + availability per status (bands 218/219).
- `src/gsv/poll.rs` + `src/gsv/client.rs` — bus polling + board forward (5 s timeout, wire-contract fix band 222).
- `deploy/systemd/telenetis.service`, `docker-compose.yml`, `scripts/telenetis-boot-verify.sh` — prod ops (band 221).

## Робочий процес (дзеркало екосистеми)

1. S0 — диск S: `df -h /s | tail -1` (≥5 GiB вільний); `git fetch`; `git status -sb`; `git log -1 --oneline`.
2. Прочитати цей файл + `docs/NEXT_SESSION_PROMPT.md` + GSV `AGENTS.md` (global-first).
3. Drain наступного банду: HANDOFF/NEXT → код → формат → тести → один commit → push.
4. Квитки: GSV `tickets.jsonl` (`gsv_tickets*`) або `gsv_telenetis_health` для keep-live-стану.
5. Пуш лише на завершення банду; **один** commit на банд.

## Правила

- **MSYS2 bash**, не PowerShell. PATH: `/c/Users/plati/.cargo/bin:/ucrt64/bin:/usr/bin:/c/msys64/usr/bin`.
- `unset RUSTUP_TOOLCHAIN` — перемагає `rust-toolchain.toml`.
- Не копіювати VDT kit сюди; не User-scope MCP; не робити цей репо оркестратором.
- `:9800` — єдиний порт репо; GSV на `:9999` і `/api/edge` — лише через токен; `:8091` не чіпати.
- Rust ≥95 %; Python = 0; `data/*`, `.env*`, `*.pem` — не в git.

## Відкриті питання (owner)

- PRODUCTS.md містить telenetis-ряди з HANDOFF/NEXT — тепер обидва файли існують; вказівник у FM poolAI (`docs/development/NEXT_SESSION_PROMPT.md`) — перевірити на poolAI дрені.
- keep-live: watchdog probe на `:9800` (band 228) — чи переходити на systemd `Restart=always` (prod-шлях band 221)?
