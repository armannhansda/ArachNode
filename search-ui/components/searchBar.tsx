import type { Dispatch, SetStateAction } from "react";

type SearchBarProps = {
  query: string;
  setQuery: Dispatch<SetStateAction<string>>;
  onSearch: (searchTerm?: string) => void;
  hasSearched: boolean;
  suggestions: string[];
  showSuggestions: boolean;
  setShowSuggestions: Dispatch<SetStateAction<boolean>>;
  activeIndex: number;
  setActiveIndex: Dispatch<SetStateAction<number>>;
};

export default function SearchBar({
  query,
  setQuery,
  onSearch,
  hasSearched,
  suggestions,
  showSuggestions,
  setShowSuggestions,
  activeIndex,
  setActiveIndex,
}: SearchBarProps) {
  return (
    <div className="relative flex-1">
      <div className="flex min-h-11 items-center rounded-full border border-[#dfe1e5] bg-white px-4 shadow-[0_1px_6px_rgba(32,33,36,0.18)] transition hover:shadow-[0_2px_8px_rgba(32,33,36,0.22)] focus-within:border-transparent focus-within:shadow-[0_2px_8px_rgba(32,33,36,0.24)]">
        <svg
          aria-hidden="true"
          className="h-5 w-5 shrink-0 text-[#9aa0a6]"
          viewBox="0 0 24 24"
          fill="currentColor"
        >
          <path d="M9.5 3a6.5 6.5 0 0 1 5.18 10.43l4.45 4.44-1.06 1.06-4.44-4.45A6.5 6.5 0 1 1 9.5 3Zm0 1.5a5 5 0 1 0 0 10 5 5 0 0 0 0-10Z" />
        </svg>
        <input
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setActiveIndex(-1);
            setShowSuggestions(e.target.value.trim().length > 0);
          }}
          onFocus={() => setShowSuggestions(false)}
          onBlur={() => setTimeout(() => setShowSuggestions(false), 150)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              if (activeIndex >= 0 && activeIndex < suggestions.length) {
                onSearch(suggestions[activeIndex]);
                return;
              }

              onSearch(query);
            }
            if (e.key === "ArrowDown")
              setActiveIndex((prev: number) =>
                prev < suggestions.length - 1 ? prev + 1 : prev
              );
            if (e.key === "ArrowUp")
              setActiveIndex((prev: number) => (prev > 0 ? prev - 1 : -1));
          }}
          className="h-11 min-w-0 flex-1 bg-transparent px-3 text-base text-[#202124] outline-none placeholder:text-[#70757a]"
          placeholder={hasSearched ? "Search" : "Search the web"}
        />

        <button
          onMouseDown={(e) => e.preventDefault()}
          onClick={() => onSearch(query)}
          className="rounded-full px-3 py-2 text-sm font-medium text-[#4285f4] transition hover:bg-[#f1f3f4]"
        >
          Search
        </button>
      </div>

      {showSuggestions && suggestions.length > 0 && (
        <div className="absolute left-0 right-0 top-full z-20 mt-1 overflow-hidden rounded-3xl border border-[#dfe1e5] bg-white py-2 shadow-[0_4px_12px_rgba(32,33,36,0.24)]">
          {suggestions.map((s: string, i: number) => (
            <div
              key={i}
              className={`cursor-pointer px-5 py-2.5 text-base text-[#202124] transition ${
                i === activeIndex ? "bg-[#f1f3f4]" : "hover:bg-[#f8fafd]"
              }`}
              onMouseDown={() => {
                onSearch(s);
              }}
            >
              {s}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
