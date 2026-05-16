Title: Implement `claim` function with secure transfers

Description:

Implement the `claim` entrypoint that allows the `recipient` (or an authorized claimer) to withdraw tokens accrued by a stream up to the current time. The function should:

- Calculate `claimable` using stored stream parameters and `now` timestamp (use `claimable_amount` helper as a basis).
- Transfer the correct amount of tokens to the recipient from the contract's escrow or by invoking token transfer if using allowance model.
- Update stream state to reflect claimed amount (reduce escrow or update `claimed` field).
- Emit `Claimed` event with `stream_id`, `recipient`, and `amount_claimed`.

Acceptance Criteria:

- Correctly computes claimable amounts across boundary cases (before start, after end, partially claimed previously).
- Prevents double-claiming and race conditions.
- Includes unit tests that simulate multiple claim calls and full-drain scenarios.

Security Considerations:

- Ensure transfer calls to token contracts handle failures and do not allow reentrancy if applicable.
- Verify caller authorization where necessary.
