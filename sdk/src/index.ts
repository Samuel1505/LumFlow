const DEFAULT_BASE = "http://localhost:3001";

export type Stream = {
  id: string;
  [key: string]: any;
};

export async function listStreams(baseUrl = DEFAULT_BASE): Promise<Stream[]> {
  const res = await fetch(`${baseUrl}/api/streams`);
  const data = await res.json();
  return data.streams || [];
}

export async function createStream(payload: any, baseUrl = DEFAULT_BASE): Promise<Stream> {
  const res = await fetch(`${baseUrl}/api/streams`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  return res.json();
}

export default {
  listStreams,
  createStream,
};
