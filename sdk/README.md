# LumFlow SDK (scaffold)

Tiny TypeScript SDK that wraps the local backend API for quick integration testing.

Usage:

```ts
import sdk from "../sdk/dist";

await sdk.createStream({
  sender: "alice",
  recipient: "bob",
  rate_per_second: 100,
});
const streams = await sdk.listStreams();
```
