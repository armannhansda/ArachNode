const express = require("express");
const redis = require("redis")
const cors = require("cors");

const PORT = 3001;
const app = express();
app.use(cors());

const client = redis.createClient();
client.connect();


app.get("/search", async (req, res) =>{
  const query = req.query.q;

  if(!query) {
    return res.json([]);
  }

  const words = query.toLowerCase().trim().split(/\s+/);
  let scores = {};

  const totalDocs = parseFloat(await client.get("total_docs")) || 1;

  for (const word of words) {
    const key = `index: ${word}`;
    const docs = await client.hGetAll(key);

    const df = Object.keys(docs).length;
    const idf = Math.log(totalDocs / (1+df));


    for (const url in docs) {
      const tf = parseFloat(docs[url]);

      if (!scores[url]) scores[url] = 0;

      scores[url] += tf*idf;
    }
  }

  let results = Object.entries(scores);

  results.sort((a,b) => b[1] - a[1]);

  res.json(results.slice(0, 20));
})

app.get("/", (req, res) => {
  res.send("hello server running")
} )

app.listen(PORT, () => {
  console.log(`server running on http://localhost:${PORT}`)
})
