---
name: bunnylol-deploy
description: Rebuilds bunnylol.rs from source and reloads its local macOS launchd service so code changes take effect on the running instance at 127.0.0.1:8123. Use this whenever the user asks to build or rebuild bunnylol, deploy it locally, restart or reload the bunnylol service, pick up their latest changes, or check whether the running bunnylol reflects their latest edits — even if they just say "deploy this", "restart the service", or "rebuild and reload" without naming bunnylol explicitly, as long as the context is this repo.
---

# Deploying bunnylol.rs locally

bunnylol.rs runs as a per-user macOS LaunchAgent, not via `cargo run`. The service
definition never changes between deploys — only the binary on disk does — so a normal
deploy is just: rebuild, kick the running process so it re-execs the new binary, verify.

Key facts about this setup (see the project's `CLAUDE.md` for the original rationale):

- Binary: `target/release/bunnylol`, built from the repo root with `cargo build --release`.
- LaunchAgent label: `com.lxxxw.bunnylol`, plist at `~/Library/LaunchAgents/com.lxxxw.bunnylol.plist`.
- The plist's `ProgramArguments` points at the fixed path above, so overwriting that file
  and restarting the service is all a normal deploy needs — the plist itself does not
  need to be touched or reloaded.
- Port/address are set via Rocket's native env vars in the plist (`ROCKET_PORT=8123`,
  `ROCKET_ADDRESS=127.0.0.1`), not `BUNNYLOL_PORT` — the binary doesn't read that var.
- Logs: `~/Library/Logs/bunnylol/stdout.log` and `stderr.log`.

## Deploy procedure

Run these three steps in order. Do not skip verification — `cargo build` succeeding
only means the code compiles, not that the new process is actually serving traffic.

### 1. Build

```bash
cd /Users/lxxxw/github/bunnylol.rs && cargo build --release
```

Check for a non-zero exit / compiler errors before continuing. A build failure means
there's nothing new to deploy — stop here and fix the code rather than restarting the
service against a stale binary.

### 2. Restart the service

```bash
launchctl kickstart -k gui/$(id -u)/com.lxxxw.bunnylol
```

`kickstart -k` kills the current process and lets launchd relaunch it from the same
`ProgramArguments` path — since step 1 just overwrote the binary at that path, the
relaunched process is the new build. You do **not** need `bootout`/`bootstrap` for a
normal deploy; that pair is only for when the plist itself changes (different port,
different env vars, different binary path — see "Full reinstall" below).

### 3. Verify the new binary is actually running

Two checks, both matter — a passing curl alone doesn't prove the *new* binary loaded
(the old process could still be answering if the restart silently no-op'd):

```bash
# Confirm state is running and note the pid changed vs. before the restart
launchctl print gui/$(id -u)/com.lxxxw.bunnylol | grep -E "pid|state"

# Confirm it's actually serving requests
curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8123/bindings
```

Expect `state = running`, a **different pid** than before the restart, and `200` from
curl. If you didn't capture the pid before restarting, that's fine — a `200` plus
`state = running` is sufficient confirmation for most purposes.

## If it doesn't come back up

Check `~/Library/Logs/bunnylol/stderr.log` for the failure — most likely causes are a
port bind conflict (something else already listening on 8123) or a runtime panic in
the new code. Once fixed, rerun step 1 and 2.

If `kickstart -k` doesn't seem to pick up the new binary (rare — would suggest launchd
state is wedged), fall back to a full reinstall instead of repeated kickstarts:

```bash
launchctl bootout gui/$(id -u)/com.lxxxw.bunnylol 2>/dev/null
launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.lxxxw.bunnylol.plist
```
