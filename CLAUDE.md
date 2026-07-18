# bunnylol.rs

Rust/Rocket web app (`src/main.rs`) that redirects `/?cmd=...` queries to registered bang-style commands.

## Running as a local service (macOS launchd)

The app doesn't read `BUNNYLOL_PORT` from `.env.example` — that var only feeds `docker-compose.yml`'s port mapping. The compiled binary is controlled by Rocket's native env vars: `ROCKET_PORT` and `ROCKET_ADDRESS`.

To run bunnylol.rs persistently on this Mac (auto-start at login, auto-restart on crash), it's installed as a per-user LaunchAgent rather than via Docker or cron:

- Binary: `target/release/bunnylol` (built via `cargo build --release`)
- LaunchAgent plist: `~/Library/LaunchAgents/com.lxxxw.bunnylol.plist`
- Bound to `127.0.0.1:8123` (localhost-only; set via `ROCKET_ADDRESS`/`ROCKET_PORT` in the plist's `EnvironmentVariables`)
- Logs: `~/Library/Logs/bunnylol/stdout.log` and `stderr.log`
- URL: http://127.0.0.1:8123/bindings

Management commands:

```bash
# status
launchctl print gui/$(id -u)/com.lxxxw.bunnylol

# stop
launchctl bootout gui/$(id -u)/com.lxxxw.bunnylol

# restart (e.g. after `cargo build --release`)
launchctl kickstart -k gui/$(id -u)/com.lxxxw.bunnylol

# reinstall from scratch after editing the plist
launchctl bootout gui/$(id -u)/com.lxxxw.bunnylol 2>/dev/null
launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.lxxxw.bunnylol.plist
```

The plist lives outside the repo (`~/Library/LaunchAgents/`), so it isn't version-controlled here — this section is the source of truth for recreating it.
