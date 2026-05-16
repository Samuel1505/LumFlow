import express from "express";
import streamsRouter from "./routes/streams";

const app = express();
app.use(express.json());

app.use("/api/streams", streamsRouter);

app.get("/health", (_req, res) => res.json({ status: "ok" }));

const PORT = process.env.BACKEND_PORT || 3001;
app.listen(Number(PORT), () => {
  // eslint-disable-next-line no-console
  console.log(`LumFlow backend running on port ${PORT}`);
});

export default app;
