# DimmOFF Pine review

TradeCraft reviewed DimmOFF's `TradingVision-pinescript-impl` branch from
`229207f089a8dd7ee89373dce1c9bc96309da7d4` through
`e7e1e98b722e59fc40e30b62bca83f5dbe47989c`. The tracked revision records
the review boundary; it does not imply that every upstream change was adopted.

The interpreter incorporates selected changes for call-expression history,
timeframe values, missing objects, string escapes, table cell updates and
position, and reducing work in higher-timeframe requests. TradeCraft retains
its own safeguards, diagnostics, daily provider history and timezone behavior.
The chart library was not updated as part of this review.

TradeCraft also implements lower-timeframe `request.security` with fetched bars,
session boundary flags, Pine label placement and persistent function-local
`var` values. Finer history is limited to 10,000 bars and nested finer requests
report an unsupported diagnostic. `request.security_lower_tf` remains unsupported.
Session flags use the selected chart timezone. As exchange calendars are not
tracked, regular-session flags use 09:30–16:00 in that timezone and do not model
holidays or early closes. Function-local persistence is isolated by call path.

The Pine source remains adapted from DimmOFF's MIT-licensed fork; see
`ATTRIBUTION.md` and the vendored MIT license. No new dependency or upstream
font or asset was adopted in this review. macOS validation requires a macOS
environment.
