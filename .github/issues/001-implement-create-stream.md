Title: Implement `create_stream` function in LumFlow contract

Description:

Implement the `create_stream` entrypoint in the Soroban `LumFlowContract` that allows a sender to create a new streaming payment. The function should:

- Accept parameters: `sender` (Address), `recipient` (Address), `rate_per_second` (u128), `start` (u64), `end` (u64), `token` (Identifier / Address for token contract).
- Validate inputs: `end > start`, `rate_per_second > 0`, and the sender authorized/signed the operation.
- Reserve or escrow the total stream amount from the sender (transfer tokens to contract or mark allowance), ensuring the contract can later transfer claimable portions to recipient.
- Persist stream metadata in on-chain storage with a unique stream id.
- Emit an event `StreamCreated` with relevant details.

Acceptance Criteria:

- `create_stream` stores a stream record and returns the stream ID.
- On success, emits `StreamCreated` event including `stream_id`, `sender`, `recipient`, `rate_per_second`, `start`, `end`, and `token`.
- Unit tests exist covering normal creation and invalid parameter cases.

Notes / Risks:

- Decide between escrow (contract holds funds) vs allowance model (contract pulls funds when claiming).
- Consider gas cost for large numbers of concurrent streams; design storage keys appropriately.
