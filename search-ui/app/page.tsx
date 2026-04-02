"use client";

import { useState } from "react";
import axios from "axios";

export default function Home() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<[string, number][]>([]);

  const handleSearch = async () => {
    const res = await axios.get(`http://localhost:3001/search?q=${query}`);
    setResults(res.data);
  };

  return (
    <div style={{ textAlign: "center", marginTop: "100px" }}>
      <h1>My Search Engine 🔍</h1>

      <input
        type="text"
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        style={{ padding: "10px", width: "300px" }}
        placeholder="Search..."
      />

      <button onClick={handleSearch} style={{ marginLeft: "10px" }}>
        Search
      </button>

      <div style={{ marginTop: "30px" }}>
        {results.map(([url, score], i) => (
          <div key={i} style={{ marginBottom: "10px" }}>
            <a href={url} target="_blank">
              {url}
            </a>
            <p>Score: {score.toFixed(3)}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
