# BatxRinth Security Policy & Architecture

## Security Architecture & Design Rules

1. **Strict Credential Isolation:**
   * Microsoft OAuth access tokens, refresh tokens, and session credentials are elements of sensitive user state.
   * Tokens are never written to log files, exported in diagnostics, or transmitted to non-essential third parties.
   * Credentials are stored in OS-level secure storage where available.

2. **Offline Local Profile Boundaries:**
   * Offline profiles strictly enforce username character sanitization (`[a-zA-Z0-9_]`, 3-16 chars).
   * Offline profiles generate local deterministic UUIDs and do not fabricate fake Mojang ownership claims, Xbox tokens, or session server signatures.
   * Offline profiles cannot be passed into authenticated online multiplayer joins (`sessionserver.mojang.com`).

3. **Network Security & TLS:**
   * BatxRinth enforces strict HTTPS connections for all remote requests.
   * Weakened TLS, disabled certificate validation, or insecure HTTP fallbacks for updates and authentication are strictly prohibited.

4. **Updater Verification:**
   * Downloaded update packages are cryptographically signed (`minisign` / Tauri updater signature validation).
   * Packages failing signature or checksum verification are rejected immediately before execution.

---

## Reporting Vulnerabilities

If you discover a potential security issue or vulnerability in **BatxRinth**, please report it responsibly by contacting the maintainers via GitHub Security Advisories on the repository:

* **Repository:** `https://github.com/BatxRinth/BatxRinth`
* Please do not open public issues for unpatched security vulnerabilities.
