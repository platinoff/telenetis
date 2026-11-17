# Промпт наступної сесії (Telenetis)

**Оновлено:** 2026-11-17 (Symbiosis P1 — HANDOFF/NEXT створено)

> **Екосистемний канон:** [`../../GSV/docs/gsv/GSV_ECOSYSTEM_SYMBIOSIS.md`](../../GSV/docs/gsv/GSV_ECOSYSTEM_SYMBIOSIS.md) — GSV «тато» / poolAI «мама». Telenetis = commutation (switching) плагін.

**Канон для відкриття:** `S:\rust\telenetis` (Cursor/OpenCode/Grok) · глобальні правила — GSV `AGENTS.md` · `GSV_AGI_PATH.md`. Читай `docs/HANDOFF_NEW_SESSION.md` (правила, стан, файли) · FM/тикети — GSV `docs/gsv/tickets.jsonl`.

**Старт сесії (порядок):**

1. `df -h /s | tail -1` (вільного диска ≥ 5 ГіБ) → у telenetis свого `cargo xtask disk` нема; під час важких збірок стежити за RAM.
2. `git fetch` → `git status -sb` → `git log -1 --oneline` (HEAD: `7efad09`).
3. Гейти: `cargo fmt -- --check` → `cargo clippy --all-targets` → `cargo test` (MSYS2 bash; `unset RUSTUP_TOOLCHAIN`; windows-gnu).
4. Лайв-служба: `:9800` (`/health`); keep-live — GSV watchdog (`telenetis_alive`); при GSV down — `gsv_telenetis_health` fail-open.

**Поточний фокус (owner 2026-11-17):**
- Екосистему переведено на канон «тато і мама» (GSV ↔ poolAI) — дивись GSV `HANDOFF_NEW_SESSION.md` та symbiosis canon.
- Telenetis: постійний стан — Mini App + board forward + presence/flows; CI-зелені ворота перед будь-яким дреном; один commit на банд.
- Кандидати смуги: prod-остаточне переведення `:9800` на systemd (`Restart=always`) з `deploy/systemd/telenetis.service`; доповнення `boot-verify` на фактичному хості; presence-fed sync тестів для `/api/board` з GSV.

**Continuity (для сесій з GSV вікна):** `абракадабра` → вибір продукту через AskQuestion / `cargo xtask products` — telenetis з'являється як окреме середовищне дерево, не як пара hardcoded.

**Кінець сесії:** один commit (код + docs) → `git push` на `master` → самарі в чат.
