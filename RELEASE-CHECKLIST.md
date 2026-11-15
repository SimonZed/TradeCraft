# Release coordination

## 0.4.0 alpha

Release scope: read-only Tradier options chains and contract charts, Pine indicator improvements, and the egui-charts update to `7a852a5`. Release source is on `codex/macos-build` and proposed in PR #9.

- [ ] Complete the comprehensive source and provenance review, including Pine imports and vendored changes. This remains follow-up work; prior reviews do not establish full clearance.
- [x] Prepare `RELEASE-NOTES-0.4.0.md` and `CHANGELOG.md` for the release.
- [ ] Resolve the missing macOS notices and remaining upstream and provider-rights questions. Windows personal and community audits passed; strict macOS checks have reported ten missing notice texts. Mac packaging records these as warnings at the owner's direction.
- [x] Candidate Windows personal and community release builds and both workspace test configurations passed; the pinned personal executable was verified against its build. Mac provider and UI tests passed on both architectures. Further interactive and broad compatibility testing remains open.
- [ ] Validate the complete interactive options workflow and broader symbol/entitlement coverage, including timestamps and missing values. A read-only live Tradier check retrieved an SPY chain with calls and puts, and one selected contract returned eight daily and 206 five-minute bars on November 14, 2026. This does not establish streaming, every entitlement, or end-to-end chart behavior.
- [x] Build and test both macOS architectures using GitHub Actions. Run 43691123701 passed both architectures at `2b3f44d`; ZIPs passed integrity checks, report version 0.4.0, contain the expected Mach-O architectures and record ten missing notice texts each. Interactive Mac validation, including Keychain and workspace restart, remains outstanding. The apps are unsigned and not notarized.
- [ ] Verify final archives and published asset checksums against the release source.

The November 14 legal and provenance review records unresolved provider authorization, upstream source and asset provenance, and other distribution questions. These require separate review; no blanket compliance clearance has been established. Preserve that detailed review privately unless its owner authorizes publication.

The first 0.4.0 GitHub Mac run exposed two options UI test failures on Apple Silicon. After test storage was isolated, both architecture jobs passed. The separate strict license workflow passed both Windows configurations, failed on Apple Silicon for ten missing notice texts and cancelled the Intel job; Mac packaging recorded those gaps as warnings.

The 0.3.0 alpha was published for Windows, Apple Silicon and Intel macOS. Its tag and assets are historical and should not be replaced by this release.
