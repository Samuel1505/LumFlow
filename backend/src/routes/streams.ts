import { Router } from "express";

const router = Router();

// In-memory example store — replace with DB in production.
const streams: Array<any> = [];

router.get("/", (_req, res) => {
  res.json({ streams });
});

router.post("/", (req, res) => {
  const payload = req.body || {};
  const id = `s_${Date.now()}`;
  const stream = { id, ...payload, created_at: Date.now() };
  streams.push(stream);
  res.status(201).json(stream);
});

router.get("/:id", (req, res) => {
  const s = streams.find((x) => x.id === req.params.id);
  if (!s) return res.status(404).json({ error: "not_found" });
  return res.json(s);
});

export default router;
