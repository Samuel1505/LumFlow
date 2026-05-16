Title: Security review and gas/size optimizations for LumFlow contract

Description:

Perform a security audit and optimize the contract for gas and wasm size. Tasks:

- Review storage layout and key choices for gas efficiency.
- Analyze potential attack vectors: replay, reentrancy (if applicable), integer overflow/underflow, unauthorized access, and denial-of-service via gas consumption.
- Ensure safe token handling: check return values of token transfer calls and handle failures gracefully.
- Reduce WASM size by removing unused code, optimizing data encodings, and using compact storage patterns.
- Add CI step to check `wasm-opt` size or compile size threshold.

Acceptance Criteria:

- Documented security findings and remediations in `SECURITY.md` or a dedicated report.
- PR with code changes that improve gas or size and accompanying benchmarks.
- CI warnings for regressions in wasm size.

Notes:

- Consider third-party audit if going to mainnet with large value flows.
