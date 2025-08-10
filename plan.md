# Backlight Controller Project Plan

## 1. Core Goal
Create an unprivileged user console tool to view and set laptop display backlight brightness safely on Arch Linux (and other systemd systems) by leveraging the systemd‑logind D‑Bus API (SetBrightness) instead of requiring root.

## 2. High‑Level Strategy
1. Learn necessary concepts (D-Bus basics → logind API → Polkit auth flow).
2. Manually experiment with the API using shell tools (no Rust yet).
3. Define CLI behavior & error model.
4. Implement minimal Rust prototype (read + set brightness via D-Bus only).
5. Add input validation & friendly messages.
6. Add optional features (list devices, set by percent, fade, etc.).
7. Document install & usage; optionally package (PKGBUILD) with udev rule guidance for fallback mode.

## 3. Learning & Doing Roadmap (Phases)
### Phase A: Concept Familiarity
- Understand D-Bus terms: bus name, object path, interface, method, signature.
- Understand difference: system bus vs session bus.
- Understand what logind provides (power/session/backlight management).
- Understand Polkit's role in authorizing privileged D-Bus calls.

### Phase B: Manual Exploration
- List available backlight devices under `/sys/class/backlight`.
- Read `max_brightness` and `brightness` manually.
- Use `busctl introspect` to view `org.freedesktop.login1.Manager` methods.
- Perform a test call to `SetBrightness` and observe success/failure.
- Monitor logind activity (`journalctl -u systemd-logind -f`).

### Phase C: Design
- Define CLI flags (e.g., `--set <0-100>`, `--device <name>`, `--list`).
- Decide output format (human readable vs quiet mode later).
- Map percent to raw value (clamp to [0, max]).
- Enumerate error categories: NoDevices, DeviceNotFound, Permission, OutOfRange, DBusFailure, Io.
- Draft user-facing error messages.

### Phase D: Prototype (Rust)
- Implement device discovery (read directory names).
- Implement reading current + max brightness.
- Implement D-Bus call to SetBrightness (single function abstraction).
- Verify by re-reading brightness.

### Phase E: Hardening
- Robust error handling with context.
- Input validation (reject >100, negative, non-numeric values, unknown device).
- Add verbose flag for debug.
- Test on invalid device to ensure clear error output.

### Phase F: Enhancements (Optional)
- Fade animation: gradual steps over time.
- Config file for default device / step size.
- Logging levels (quiet/json output for scripts).
- Direct sysfs fallback (only if user has permissions) with explicit opt-in.

### Phase G: Distribution
- Write concise README (purpose, safety, usage, permission model).
- Create PKGBUILD (ship binary + optional udev rule file if you choose to support direct mode).
- Versioning & semantic tags.

## 4. Actionable Checklist
Learning:
- [x] Skim D-Bus tutorial basics (messages, bus types).
- [x] Read `man org.freedesktop.login1` (focus on SetBrightness).
- [ ] Skim Polkit `man polkit` & identify where actions are defined.

Manual Experiments:
- [ ] `ls /sys/class/backlight` to list devices.
- [ ] `cat /sys/class/backlight/<dev>/max_brightness` record value.
- [ ] `busctl list | grep login1` confirm service.
- [ ] `busctl introspect org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager | grep SetBrightness` confirm signature.
- [ ] Perform safe call: `busctl call org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager SetBrightness ssi backlight <dev> <raw>`.
- [ ] Verify brightness changed (read file again or visually).
- [ ] Trigger a failure with wrong device name; capture error message.

Design & Implementation:
- [ ] Finalize CLI flags.
- [ ] Define internal error enum mapping to exit codes.
- [ ] Prototype D-Bus setter function.
- [ ] Add verification read-back.
- [ ] Implement percent to raw conversion & clamp.
- [ ] Add device listing command.

Quality:
- [ ] Add simple tests for percent→raw conversion logic.
- [ ] Run `cargo clippy` and address warnings.
- [ ] Run `cargo fmt`.

Docs & Packaging:
- [ ] Write README (overview, safety, usage examples, troubleshooting).
- [ ] Add section: "If D-Bus fails: diagnosing Polkit / permissions.".
- [ ] Consider PKGBUILD (optional).

Optional Enhancements:
- [ ] Fade/transition feature.
- [ ] JSON output mode.
- [ ] Config file support (~/.config/backlight-controller/config.toml).
- [ ] Direct sysfs fallback (warn about permissions).

## 5. Key Commands Reference (No Code)
```
# List services containing 'login1'
busctl list | grep login1

# Introspect logind manager object (full)
busctl introspect org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager

# Isolate SetBrightness line
grep SetBrightness <(busctl introspect org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager)

# Call SetBrightness (replace <dev>, <raw>)
busctl call org.freedesktop.login1 /org/freedesktop/login1 org.freedesktop.login1.Manager SetBrightness ssi backlight <dev> <raw>

# Monitor D-Bus traffic for login1
busctl monitor org.freedesktop.login1

# Follow logind journal entries
journalctl -u systemd-logind -f

# View polkit actions referencing login1
grep -i login1 /usr/share/polkit-1/actions/*.policy
```

## 6. Essential Resources / Links
- D-Bus tutorial overview: https://dbus.freedesktop.org/doc/dbus-tutorial.html
- D-Bus specification (types/signatures reference): https://dbus.freedesktop.org/doc/dbus-specification.html
- systemd-logind D-Bus API (man page): https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html
- systemd-logind general info: https://www.freedesktop.org/wiki/Software/systemd/logind/
- Polkit manual: https://www.freedesktop.org/software/polkit/docs/latest/polkit.8.html
- Polkit actions directory (local files): /usr/share/polkit-1/actions/
- Example tool (for behavior inspiration): brightnessctl (Arch package; not code-copied, just conceptual)

## 7. Error Categories (Planned Mapping)
| Category | Trigger | User Message (draft) |
|----------|---------|----------------------|
| NoDevices | Directory empty | "No backlight devices found." |
| DeviceNotFound | Unknown name | "Device '<name>' not found; available: ..." |
| Permission | D-Bus AccessDenied | "Not authorized to set brightness (check active session / polkit)." |
| OutOfRange | Percent >100 or raw > max | "Requested value outside 0-100%." |
| DBusFailure | Other D-Bus error | "D-Bus error: <detail>." |
| Io | File read error | "Failed to read brightness info: <detail>." |

## 8. Validation Steps (Manual)
1. Show current brightness: run prototype with no args (shows device + percent).
2. Set to 50%; verify UI brightness and file value changed.
3. Set to 0% (screen may get very dim) then restore to previous value.
4. Attempt invalid device; confirm clear error.
5. Run while another tool changes brightness; ensure final reported value matches reality.
6. Run from a non-active session (e.g., SSH) to observe permission denial (if desired).

## 9. Future Enhancements Ideas
- Multi-backlight selection (e.g., keyboards, eDP + dedicated GPU).
- Daemon mode for smooth transitions with signals.
- Scripting hooks when brightness changes.
- Integrate with power profiles (lower brightness on battery).

## 10. Glossary
- Bus name: Identifies the D-Bus service (e.g., org.freedesktop.login1).
- Object path: Hierarchical path to object (e.g., /org/freedesktop/login1).
- Interface: Namespaced set of methods (e.g., org.freedesktop.login1.Manager).
- Method: Remote callable function (e.g., SetBrightness).
- Signature: Type sequence for parameters (e.g., ssi = string, string, int32).
- Polkit: Authorization framework mediating privileged operations.

---
Keep this plan updated: check tasks off, add discoveries, and adjust error handling notes as you test.
