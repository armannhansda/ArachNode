import { useEffect, useRef, useState } from "react";
import { fetchSearchResult, fetchSuggestions } from "@/services/searchSearvice";

type SearchResult = {
  url: string;
  title: string;
  description: string;
  score?: number;
};

export function useSearch() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [hasSearched, setHasSearched] = useState(false);
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [activeIndex, setActiveIndex] = useState(-1);
  const [lastQuery, setLastQuery] = useState("");

  const observerRef = useRef<HTMLDivElement | null>(null);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const loadingRef = useRef(false);
  const searchRequestIdRef = useRef(0);
  const suggestionRequestIdRef = useRef(0);

  useEffect(() => {
    loadingRef.current = loading;
  }, [loading]);

  const resetSearchState = () => {
    setResults([]);
    setPage(1);
    setHasMore(true);
    setError("");
  };

  const search = async (q: string, newPage = 1, reset = false) => {
    const trimmedQuery = q.trim();

    if (!trimmedQuery || (loadingRef.current && !reset)) {
      return;
    }

    const requestId = searchRequestIdRef.current + 1;
    searchRequestIdRef.current = requestId;

    try {
      setLoading(true);
      setError("");

      const data = await fetchSearchResult(trimmedQuery, newPage);

      if (requestId !== searchRequestIdRef.current) {
        return;
      }

      setResults((prev) =>
        reset ? data.results : [...prev, ...data.results]
      );
      setHasMore(Boolean(data.hasMore));
      setPage(newPage);
    } catch {
      setError("Search failed");
    } finally {
      if (requestId === searchRequestIdRef.current) {
        setLoading(false);
      }
    }
  };

  //debounce search
  useEffect(() => {
    if (!query.trim()) {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
      return;
    }

    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    debounceRef.current = setTimeout(() => {
      const trimmedQuery = query.trim();

      setLastQuery(trimmedQuery);
      setHasSearched(true);
      resetSearchState();
      void search(trimmedQuery, 1, true);
    }, 600);

    return () => {
      if (debounceRef.current) {
        clearTimeout(debounceRef.current);
      }
    };
  }, [query]);

  //suggetions
  useEffect(()=>{
    if(!query.trim()){
      setSuggestions([]);
      setShowSuggestions(false);
      setActiveIndex(-1);
      return;
    }

    const timeout = setTimeout(async () => {
      const requestId = suggestionRequestIdRef.current + 1;
      suggestionRequestIdRef.current = requestId;

      try {
        const data = await fetchSuggestions(query.trim());

        if (requestId !== suggestionRequestIdRef.current) {
          return;
        }

        setSuggestions(data);
      } catch {
        if (requestId !== suggestionRequestIdRef.current) {
          return;
        }

        setSuggestions([]);
        setShowSuggestions(false);
      }
    }, 300);

    return () => clearTimeout(timeout);
  }, [query]);

  useEffect(() => {
    const el = observerRef.current;

    if(!el || !hasMore || !lastQuery) return;

    const observer = new IntersectionObserver((entries) => {
      if(entries[0]?.isIntersecting && !loadingRef.current){
        void search(lastQuery, page + 1);
      }
    });
    
    observer.observe(el);
    return () => observer.disconnect();
  }, [page, hasMore, lastQuery]);

  const submitSearch = (searchTerm = query) => {
    const trimmedQuery = searchTerm.trim();

    if (!trimmedQuery) {
      setSuggestions([]);
      setShowSuggestions(false);
      return;
    }

    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    setQuery(trimmedQuery);
    setLastQuery(trimmedQuery);
    setHasSearched(true);
    setSuggestions([]);
    setShowSuggestions(false);
    setActiveIndex(-1);
    resetSearchState();
    void search(trimmedQuery, 1, true);
  };

  return {
    query,
    setQuery,
    results,
    error,
    suggestions,
    showSuggestions,
    setShowSuggestions,
    loading,
    hasMore,
    hasSearched,
    activeIndex,
    setActiveIndex,
    observerRef,
    submitSearch,
  };
}
