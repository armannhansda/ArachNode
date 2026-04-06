"use client";

import { useState } from "react";
import axios from "axios";

type SearchResult = {
  url: string;
  title: string;
  description: string;
  score: number;
};

export default function Home() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [error, setError] = useState("");

  const handleSearch = async () => {
    try {
      setError("");
      const res = await axios.get(`http://localhost:3001/search?q=${query}`);
      setResults(res.data);
    } catch (err) {
      const message =
        axios.isAxiosError(err) && err.response?.data?.details
          ? err.response.data.details
          : "Search failed";

      setResults([]);
      setError(message);
    }
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
        {error ? <p style={{ color: "red" }}>{error}</p> : null}
        {results.map((result, i) => (
          <div key={i} style={{ marginBottom: "20px" }}>
            <a href={result.url} target="_blank" rel="noreferrer">
              {result.title || result.url}
            </a>
            <p style={{ margin: "6px 0" }}>{result.url}</p>
            <p style={{ margin: "6px 0" }}>{result.description}</p>
            <p>Score: {result.score.toFixed(3)}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
