type SearchResult = {
  url: string;
  title?: string;
  description?: string;
  score?: number;
};

type ResultsListProps = {
  results: SearchResult[];
};

export default function ResultsList({ results }: ResultsListProps) {
  return (
    <div className="space-y-7">
      {results.map((r: any) => (
        <div key={r.url} className="max-w-[690px]">
          <div className="min-w-0">
            <div className="mb-1 flex min-w-0 items-center gap-3">
              <img
                src={`https://www.google.com/s2/favicons?domain=${r.url}`}
                alt=""
                className="h-7 w-7 shrink-0 rounded-full bg-[#f1f3f4] p-1.5"
              />
              <div className="min-w-0">
                <p className="truncate text-sm leading-5 text-[#202124]">
                  {r.title || r.url}
                </p>
                <p className="truncate text-xs leading-4 text-[#4d5156]">
                  {r.url}
                </p>
              </div>
            </div>

            <a
              href={r.url}
              target="_blank"
              rel="noreferrer"
              className="block text-xl leading-6 text-[#1a0dab] hover:underline"
            >
              {r.title || r.url}
            </a>

            {r.description && (
              <p className="mt-1 text-sm leading-6 text-[#4d5156]">
                {r.description}
              </p>
            )}

            {typeof r.score === "number" && (
              <p className="mt-1 text-xs text-[#70757a]">
                Score: {r.score.toFixed(3)}
              </p>
            )}
          </div>
        </div>
      ))}
    </div>
  );
}
