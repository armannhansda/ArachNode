"use client";

import SearchBar from "../components/searchBar";
import ResultsList from "../components/resultList";
import { useSearch } from "../hooks/useSearch";

export default function Home() {
  const {
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
  } = useSearch();

  return (
    <div className="min-h-screen bg-white text-[#202124]">
      <div className="mx-auto min-h-screen w-full px-4 sm:px-6">
        {hasSearched ? (
          <div className="space-y-6">
            <div className="sticky top-0 z-10 -mx-4 border-b border-[#dadce0] bg-white px-4 py-5 sm:-mx-6 sm:px-6">
              <div className="flex max-w-5xl flex-col gap-4 sm:flex-row sm:items-center sm:gap-9">
                <h1 className="shrink-0 text-3xl font-normal tracking-tight text-[#4285f4]">
                  <span className="text-[#4285f4]">A</span>
                  <span className="text-[#ea4335]">r</span>
                  <span className="text-[#fbbc05]">a</span>
                  <span className="text-[#4285f4]">c</span>
                  <span className="text-[#34a853]">h</span>
                  <span className="text-[#ea4335]">Node</span>
                </h1>

                <div className="w-full max-w-[690px]">
                  <SearchBar
                    query={query}
                    setQuery={setQuery}
                    onSearch={submitSearch}
                    hasSearched={hasSearched}
                    suggestions={suggestions}
                    showSuggestions={showSuggestions}
                    setShowSuggestions={setShowSuggestions}
                    activeIndex={activeIndex}
                    setActiveIndex={setActiveIndex}
                  />
                </div>
              </div>
            </div>

            <div className="mx-auto w-full max-w-5xl">
              <p className="max-w-[690px] text-sm text-[#70757a] sm:ml-[168px]">
                About {results.length} result{results.length === 1 ? "" : "s"}
              </p>
            </div>

            {loading && results.length === 0 && (
              <p className="mx-auto w-full max-w-5xl text-sm text-[#70757a] sm:pl-[168px]">
                Searching...
              </p>
            )}

            {error && (
              <p className="mx-auto w-full max-w-5xl text-sm text-[#d93025] sm:pl-[168px]">
                {error}
              </p>
            )}

            {!loading && !error && (
              <div className="mx-auto w-full max-w-5xl sm:pl-[168px]">
                <ResultsList results={results} />
              </div>
            )}

            {hasSearched && hasMore && (
              <div
                ref={observerRef}
                className="mx-auto w-full max-w-5xl py-6 text-sm text-[#70757a] sm:pl-[168px]"
              >
                {loading ? "Loading more..." : ""}
              </div>
            )}
          </div>
        ) : (
          <div className="flex min-h-screen items-start justify-center pt-[18vh]">
            <div className="w-full max-w-3xl text-center">
              <h1 className="text-6xl font-normal tracking-tight sm:text-7xl">
                <span className="text-[#4285f4]">A</span>
                <span className="text-[#ea4335]">r</span>
                <span className="text-[#fbbc05]">a</span>
                <span className="text-[#4285f4]">c</span>
                <span className="text-[#34a853]">h</span>
                <span className="text-[#ea4335]">Node</span>
              </h1>
              <p className="mt-3 text-sm text-[#70757a]">
                Search the web
              </p>

              <div className="mx-auto mt-8 max-w-2xl">
                <SearchBar
                  query={query}
                  setQuery={setQuery}
                  onSearch={submitSearch}
                  hasSearched={hasSearched}
                  suggestions={suggestions}
                  showSuggestions={showSuggestions}
                  setShowSuggestions={setShowSuggestions}
                  activeIndex={activeIndex}
                  setActiveIndex={setActiveIndex}
                />
              </div>

              {loading && <p className="mt-6 text-sm text-[#70757a]">Searching...</p>}
              {error && <p className="mt-6 text-sm text-[#d93025]">{error}</p>}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
