# Product

A local browser API for shell-capable agents. One native Rust CLI searches,
reads, extracts, interacts, and makes browser-authenticated requests.
Users explicitly choose existing Chrome or a separate persistent profile.
Existing Chrome requires Chrome 144+, remote debugging, and approval.
The macOS/Linux helper reuses that approved connection until disconnect or loss.
Browser consent grants broad access, not task-specific authorization.
Websites still control access. Never imply a CAPTCHA or login bypass.

Released baseline: 0.2.0. Core redesign: unpublished development build.
Performance evidence: docs/core-redesign.md and its raw benchmark JSON files.
Do not imply Cargo/npm installs include the unreleased redesign.
