# TradeCraft 0.4.0 alpha — release candidate

TradeCraft 0.4.0 adds a read-only options workflow, expands custom Pine indicator support and updates the charting library with new indicator and chart controls.

## Options chains and contract charts

With Tradier selected, open **Options** from a chart or a watchlist symbol. Choose an expiration, switch between calls and puts, and inspect available prices, volume, open interest, implied volatility and Greeks. Select a contract's last price to open its chart in a new tab.

Choose which optional columns to display and keep those preferences between sessions. The chain shows the underlying price among the strikes and presents per-share prices alongside their ×100 amounts for convenient comparison.

The chain is a manually refreshed snapshot. Availability depends on the Tradier account and provider; the ×100 display is before fees and adjusted contracts may differ. A live SPY chain and one contract's daily and five-minute history were checked; the complete interactive workflow and broader coverage still need verification.

## Pine indicators

Pine indicators gain support for selected additional language features, including finer-timeframe requests, session flags, persistent function-local values and improved handling of historical prior-day values. The update also includes selected improvements from DimmOFF's fork while retaining TradeCraft's editor, safeguards and provider-backed daily history.

Pine remains a subset of the language. Session flags follow the chart timezone and use regular-session hours without an exchange holiday or early-close calendar; finer history is bounded. See the README and project documentation for compatibility and data limitations.

## Charts and indicators

The charting library has been updated with improvements to built-in studies, drawings, appearance and navigation. Highlights include RSI divergence options, additional auto-anchored VWAP bands, expanded Technical Ratings settings, scale appearance controls and drawing settings. TradeCraft's provider integration, Pine work and chart-position behavior are retained.

## Release status

TradeCraft remains a read-only alpha. The Windows personal and community editions maintain separate provider preferences. Mac builds and interactive platform behavior require validation for this revision. Provider distribution rights, upstream provenance and third-party notice questions remain documented in the project records; no blanket compliance clearance is implied.
