Title: Add comprehensive unit and integration tests for LumFlow contract

Description:

Create test coverage for the LumFlow contract that verifies correctness across scenarios:

- Unit tests for helper functions (`claimable_amount`, storage helpers).
- End-to-end tests that deploy the contract to the Soroban test environment (or use the local VM) and exercise the main flows: create -> claim -> pause/resume -> cancel.
- Edge cases: overlapping pauses, backdated timestamps, extremely large rates, token transfer failures, and unauthorized calls.

Acceptance Criteria:

- Tests run with `cargo test` and pass locally in CI.
- Include a script or CI job step to run contract tests and integration tests against Soroban testnet/local node.
- Use deterministic accounts and fixed timestamps where possible to avoid flakiness.

Notes:

- Consider using `soroban-env-guest` test helpers or existing Soroban testing frameworks.
