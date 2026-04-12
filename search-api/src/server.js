const express = require("express");
const cors = require("cors");

const { getPagesCollection } = require("../utils/mongoDB");

const PORT = process.env.PORT || 3001;
const app = express();

app.use(cors());
app.use(express.json());

function escapeRegex(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function countMatches(text, word) {
  if (!text) {
    return 0;
  }

  const matches = text.match(new RegExp(escapeRegex(word), "gi"));
  return matches ? matches.length : 0;
}

function scorePage(page, words) {
  return words.reduce((score, word) => {
    const titleMatches = countMatches(page.title, word);
    const descriptionMatches = countMatches(page.description, word);
    const contentMatches = countMatches(page.content, word);

    return score + titleMatches * 5 + descriptionMatches * 3 + contentMatches;
  }, 0);
}

app.get("/search", async (req, res) => {
  const query = req.query.q?.trim();
  const page = parseInt(req.query.page) || 1;
  const limit = parseInt(req.query.limit) || 10;
  const startTime = Date.now();

  if (!query) {
    return res.json({
      results: [],
      page: 1,
      totalPages: 0,
      totalResults: 0,
    });
  }

  try {
    const pages = await getPagesCollection();
    const words = query.toLowerCase().split(/\s+/).filter(Boolean);

    const filters = words.map((word) => ({
      $or: [
        { title: { $regex: escapeRegex(word), $options: "i" } },
        { description: { $regex: escapeRegex(word), $options: "i" } },
        { content: { $regex: escapeRegex(word), $options: "i" } },
      ],
    }));

    // Step 1: Fetch larger pool (important for scoring accuracy)
    const matches = await pages
      .find({ $and: filters })
      .project({ _id: 0, url: 1, title: 1, description: 1, content: 1 })
      .limit(500) // bigger pool improves ranking
      .toArray();

    // Step 2: Score + Sort
    const scoredResults = matches
      .map((page) => ({
        url: page.url,
        title: page.title,
        description: page.description,
        score: scorePage(page, words),
      }))
      .filter((page) => page.score > 0)
      .sort((a, b) => b.score - a.score);

    const totalResults = scoredResults.length;
    const totalPages = Math.ceil(totalResults / limit);

    // Step 3: Paginate AFTER sorting
    const start = (page - 1) * limit;
    const paginatedResults = scoredResults.slice(start, start + limit);

    const hasMore = page < totalPages;

    // time taken
    const timeTaken = Date.now() - startTime;

    res.json({
      results: paginatedResults,
      page,
      totalPages,
      totalResults,
      hasMore,
      timeTaken,
    });
  } catch (error) {
    console.error("Search failed:", error);
    res.status(500).json({
      error: "Failed to search pages",
      details: error.message,
    });
  }
});

app.get("/suggetion", async (req, res) =>{
  const query = req.query.q?.trim();

  if(!query) return res.json([]);

  try {
    
    const pages = await getPagesCollection();

    const suggetions = await pages
      .find(
        {title: {$regex:query, $options: "i"}},
        {projection: {title: 1, _id:0}}
      )
      .limit(5)
      .toArray()
    
    const titles = suggetions.map((s) => s.title);

    res.json(titles);
  } catch (error) {
    res.status(500).json([]);
  }
});



app.get("/", async (_req, res) => {
  try {
    await getPagesCollection();
    res.send("search api running");
  } catch (error) {
    console.error("MongoDB connection failed:", error);
    res.status(500).send("search api failed to connect to mongodb");
  }
});

app.listen(PORT, () => {
  console.log(`server running on http://localhost:${PORT}`);
});
