# Release coordination

## 0.4.0 alpha — in preparation

Release scope: read-only Tradier options chains and contract charts, Pine indicator improvements, and the egui-charts update to `7a852a5`. The candidate source is committed on `codex/macos-build` and proposed in PR #9. The current 0.4.0 version is a release candidate, not a published release.

- [ ] Review the complete source diff, including untracked application files, and reconcile the changes from all project chats. Preserve the TradeCraft-specific egui-charts changes and reconcile Pine imports with attribution and review records.
- [ ] Complete `RELEASE-NOTES-0.4.0.md` and update `CHANGELOG.md` for the final release. Verify the notes against the final source and packages.
- [ ] Run the dependency license audit for Windows personal and community and both macOS targets on the final lockfile. Review any changes and include the generated notices. The strict macOS checks previously reported missing notices; the packaging script records these as warnings at the owner's direction. The open upstream and provider-rights issues are not resolved by a successful build.
- [ ] Run the appropriate workspace, options, Pine and vendored chart tests on the final source. Verify the community and personal Windows release builds; the personal build must update and verify `target/personal/tradecraft.exe`.
- [ ] Validate the complete interactive options workflow and broader symbol/entitlement coverage, including timestamps and missing values. A read-only live Tradier check retrieved an SPY chain with calls and puts, and one selected contract returned eight daily and 206 five-minute bars on November 14, 2026. This does not establish streaming, every entitlement, or end-to-end chart behavior.
- [x] Build and test both macOS architectures for the candidate using GitHub Actions. Run 43686509325 passed both architectures at `eae3a82`; `8ee1f94` has an identical source tree. Downloaded ZIPs passed integrity checks, report version 0.4.0, contain the expected Mach-O architectures and record ten missing notice texts each. Interactive Mac validation, including Keychain and workspace restart, remains outstanding. Rebuild packages if release contents change.
- [ ] Check the final Windows and macOS archives, version, executable identities and notices against the same release source. Follow the review and release process before publishing.

The November 14 legal and provenance review records unresolved provider authorization, upstream source and asset provenance, and other distribution questions. These require separate review; no blanket compliance clearance has been established. Preserve that detailed review privately unless its owner authorizes publication.

The first 0.4.0 GitHub Mac run exposed two options UI test failures on Apple Silicon. After test storage was isolated, both architecture jobs passed. The separate strict license workflow passed both Windows configurations, failed on Apple Silicon for ten missing notice texts and cancelled the Intel job; Mac packaging recorded those gaps as warnings.

The 0.3.0 alpha was published for Windows, Apple Silicon and Intel macOS. Its tag and assets are historical and should not be replaced by this release.
