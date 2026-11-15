# Release coordination

## 0.4.0 alpha — in preparation

Release scope: read-only Tradier options chains and contract charts, Pine indicator improvements, and the egui-charts update to `7a852a5`. The source is on `codex/macos-build`; much of the release work is uncommitted. The current 0.4.0 version is a release candidate, not a published release.

- [ ] Review the complete source diff, including untracked application files, and reconcile the changes from all project chats. Preserve the TradeCraft-specific egui-charts changes and reconcile Pine imports with attribution and review records.
- [ ] Complete `RELEASE-NOTES-0.4.0.md` and update `CHANGELOG.md` for the final release. Verify the notes against the final source and packages.
- [ ] Run the dependency license audit for Windows personal and community and both macOS targets on the final lockfile. Review any changes and include the generated notices. The strict macOS checks previously reported missing notices; the packaging script records these as warnings at the owner's direction. The open upstream and provider-rights issues are not resolved by a successful build.
- [ ] Run the appropriate workspace, options, Pine and vendored chart tests on the final source. Verify the community and personal Windows release builds; the personal build must update and verify `target/personal/tradecraft.exe`.
- [ ] Validate the complete interactive options workflow and broader symbol/entitlement coverage, including timestamps and missing values. A read-only live Tradier check retrieved an SPY chain with calls and puts, and one selected contract returned eight daily and 206 five-minute bars on November 14, 2026. This does not establish streaming, every entitlement, or end-to-end chart behavior.
- [ ] Build and test both macOS architectures for this revision using GitHub Actions. Inspect the app bundles and notice inventories; the prior 0.3.0 builds do not validate 0.4.0. Interactive Mac validation, including Keychain and workspace restart, remains outstanding.
- [ ] Check the final Windows and macOS archives, version, executable identities and notices against the same release source. Follow the review and release process before publishing.

The November 14 legal and provenance review records unresolved provider authorization, upstream source and asset provenance, and other distribution questions. These require separate review; no blanket compliance clearance has been established. Preserve that detailed review privately unless its owner authorizes publication.

The first 0.4.0 GitHub Mac run exposed two options UI test failures on Apple Silicon. The tests were adjusted to use isolated storage and are being checked again; do not infer a Mac app failure or a successful Mac validation from that initial result alone.

The 0.3.0 alpha was published for Windows, Apple Silicon and Intel macOS. Its tag and assets are historical and should not be replaced by this release.
