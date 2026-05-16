Title: Add `pause_stream`, `resume_stream`, and `cancel_stream` lifecycle operations

Description:

Add lifecycle management for streams to support pausing, resuming, and cancelling streams. Requirements:

- `pause_stream(stream_id)` should pause accrual at current timestamp; record `paused_at` and update internal counters so that claimable amount stops increasing while paused.
- `resume_stream(stream_id)` should resume accrual from the resume time; adjust bookkeeping so accrual continues correctly and total amount remains bounded by original schedule.
- `cancel_stream(stream_id)` should finalize the stream: compute claimable for recipient, transfer claimable amount, and refund unstreamed amount to sender; emit `StreamCancelled` event.

Acceptance Criteria:

- Correct state transitions with authorization checks (only sender can pause/resume/cancel unless explicit permission granted).
- Tests validating pause/resume behavior across multiple pauses and cancels.
- Events emitted: `StreamPaused`, `StreamResumed`, `StreamCancelled` with clear metadata.

Edge Cases:

- Pausing before start should be a no-op or allowed with defined semantics.
- Cancelling a fully-claimed stream should simply error or be idempotent.
