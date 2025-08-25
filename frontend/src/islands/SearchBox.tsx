import { useState, useEffect, useRef } from 'react';
import { Search, X } from 'lucide-react';

interface SearchArticleResult {
  id: string;
  title: string;
  subtitle?: string;
  excerpt: string;
  slug?: string;
  author: {
    id: string;
    username: string;
    display_name?: string;
    avatar_url?: string;
  };
  claps_count: number;
  reading_time_minutes: number;
  published_at: string;
  relevance_score: number;
}

interface SearchUserResult {
  id: string;
  username: string;
  display_name?: string;
  bio?: string;
  avatar_url?: string;
  followers_count: number;
  articles_count: number;
  is_verified: boolean;
  relevance_score: number;
}

interface SearchResponse {
  query: string;
  total_results: number;
  results: {
    articles: SearchArticleResult[];
    users: SearchUserResult[];
    publications: any[];
    tags: any[];
  };
  suggestions: string[];
}

export default function SearchBox() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [searchResponse, setSearchResponse] = useState<SearchResponse | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const searchRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  // Close search on outside click
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (searchRef.current && !searchRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  // Focus input when opened
  useEffect(() => {
    if (isOpen && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isOpen]);

  // Debounced search
  useEffect(() => {
    if (!query.trim()) {
      setSearchResponse(null);
      return;
    }

    const timeoutId = setTimeout(async () => {
      setIsLoading(true);
      try {
        const response = await fetch(`/api/v1/search?q=${encodeURIComponent(query)}`);
        if (response.ok) {
          const data: SearchResponse = await response.json();
          setSearchResponse(data);
        } else {
          console.error('Search failed:', response.statusText);
          setSearchResponse(null);
        }
      } catch (error) {
        console.error('Search failed:', error);
        setSearchResponse(null);
      } finally {
        setIsLoading(false);
      }
    }, 300);

    return () => clearTimeout(timeoutId);
  }, [query]);

  return (
    <div className="relative w-full" ref={searchRef}>
      {/* Search Input Button */}
      <button
        onClick={() => setIsOpen(true)}
        className="w-full flex items-center space-x-3 px-4 py-2 bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-full hover:border-gray-300 dark:hover:border-gray-600 transition-colors text-left"
        aria-label="Search"
      >
        <Search className="h-4 w-4 text-gray-400" />
        <span className="text-gray-500 dark:text-gray-400 text-sm">Search articles, authors, topics...</span>
      </button>

      {/* Search Modal */}
      {isOpen && (
        <div className="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm">
          <div className="flex items-start justify-center pt-[10vh] px-4">
            <div className="w-full max-w-2xl bg-white dark:bg-gray-800 rounded-lg shadow-xl border dark:border-gray-700">
              {/* Search Input */}
              <div className="flex items-center border-b dark:border-gray-700 px-4">
                <Search className="h-5 w-5 text-gray-400" />
                <input
                  ref={inputRef}
                  type="text"
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  placeholder="Search articles, authors, topics..."
                  className="flex-1 px-4 py-4 bg-transparent border-none outline-none text-gray-900 dark:text-white placeholder-gray-500"
                />
                <button
                  onClick={() => setIsOpen(false)}
                  className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                >
                  <X className="h-5 w-5" />
                </button>
              </div>

              {/* Search Results */}
              <div className="max-h-96 overflow-y-auto">
                {isLoading && (
                  <div className="p-4 text-center text-gray-500">
                    <div className="animate-spin h-6 w-6 border-2 border-blue-600 border-t-transparent rounded-full mx-auto"></div>
                  </div>
                )}

                {!isLoading && query && searchResponse && searchResponse.total_results === 0 && (
                  <div className="p-4 text-center text-gray-500">
                    No results found for "{query}"
                  </div>
                )}

                {searchResponse && (
                  <div>
                    {/* Articles Section */}
                    {searchResponse.results.articles.length > 0 && (
                      <div>
                        <div className="px-4 py-2 bg-gray-50 dark:bg-gray-700 text-sm font-medium text-gray-600 dark:text-gray-300">
                          Articles ({searchResponse.results.articles.length})
                        </div>
                        {searchResponse.results.articles.map((article) => (
                          <a
                            key={article.id}
                            href={article.slug ? `/article/${article.slug}` : `/article/${article.id}`}
                            className="block p-4 hover:bg-gray-50 dark:hover:bg-gray-700 border-b dark:border-gray-700"
                            onClick={() => setIsOpen(false)}
                          >
                            <div className="flex items-start space-x-3">
                              <img
                                src={article.author.avatar_url ? 
                                  (article.author.avatar_url.startsWith('/') ? 
                                    `http://localhost:3001${article.author.avatar_url}` : 
                                    article.author.avatar_url
                                  ) : '/default-avatar.png'
                                }
                                alt={article.author.display_name || article.author.username}
                                className="w-8 h-8 rounded-full bg-gray-200 dark:bg-gray-700"
                                onError={(e) => {
                                  (e.target as HTMLImageElement).src = 'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMzIiIGhlaWdodD0iMzIiIHZpZXdCb3g9IjAgMCAzMiAzMiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMTYiIGN5PSIxNiIgcj0iMTYiIGZpbGw9IiNFNUU3RUIiLz4KPGNpcmNsZSBjeD0iMTYiIGN5PSIxMyIgcj0iNSIgZmlsbD0iIzlDQTNBRiIvPgo8cGF0aCBkPSJNNiAyNmMwLTUuNTIzIDQuNDc3LTEwIDEwLTEwczEwIDQuNDc3IDEwIDEwIiBmaWxsPSIjOUNBM0FGIi8+Cjwvc3ZnPgo=';
                                }}
                              />
                              <div className="flex-1 min-w-0">
                                <h3 className="text-sm font-medium text-gray-900 dark:text-white truncate">
                                  {article.title}
                                </h3>
                                {article.subtitle && (
                                  <p className="text-xs text-gray-600 dark:text-gray-400 truncate">
                                    {article.subtitle}
                                  </p>
                                )}
                                <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mt-1">
                                  {article.excerpt}
                                </p>
                                <div className="flex items-center justify-between mt-2 text-xs text-gray-400">
                                  <span>by {article.author.display_name || article.author.username}</span>
                                  <div className="flex items-center space-x-2">
                                    <span>{article.claps_count} claps</span>
                                    <span>•</span>
                                    <span>{article.reading_time_minutes} min read</span>
                                  </div>
                                </div>
                              </div>
                            </div>
                          </a>
                        ))}
                      </div>
                    )}

                    {/* Users Section */}
                    {searchResponse.results.users.length > 0 && (
                      <div>
                        <div className="px-4 py-2 bg-gray-50 dark:bg-gray-700 text-sm font-medium text-gray-600 dark:text-gray-300">
                          Users ({searchResponse.results.users.length})
                        </div>
                        {searchResponse.results.users.map((user) => (
                          <a
                            key={user.id}
                            href={`/@${user.username}`}
                            className="block p-4 hover:bg-gray-50 dark:hover:bg-gray-700 border-b dark:border-gray-700"
                            onClick={() => setIsOpen(false)}
                          >
                            <div className="flex items-start space-x-3">
                              <img
                                src={user.avatar_url ? 
                                  (user.avatar_url.startsWith('/') ? 
                                    `http://localhost:3001${user.avatar_url}` : 
                                    user.avatar_url
                                  ) : '/default-avatar.png'
                                }
                                alt={user.display_name || user.username}
                                className="w-8 h-8 rounded-full bg-gray-200 dark:bg-gray-700"
                                onError={(e) => {
                                  (e.target as HTMLImageElement).src = 'data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMzIiIGhlaWdodD0iMzIiIHZpZXdCb3g9IjAgMCAzMiAzMiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPGNpcmNsZSBjeD0iMTYiIGN5PSIxNiIgcj0iMTYiIGZpbGw9IiNFNUU3RUIiLz4KPGNpcmNsZSBjeD0iMTYiIGN5PSIxMyIgcj0iNSIgZmlsbD0iIzlDQTNBRiIvPgo8cGF0aCBkPSJNNiAyNmMwLTUuNTIzIDQuNDc3LTEwIDEwLTEwczEwIDQuNDc3IDEwIDEwIiBmaWxsPSIjOUNBM0FGIi8+Cjwvc3ZnPgo=';
                                }}
                              />
                              <div className="flex-1 min-w-0">
                                <div className="flex items-center space-x-1">
                                  <h3 className="text-sm font-medium text-gray-900 dark:text-white truncate">
                                    {user.display_name || user.username}
                                  </h3>
                                  {user.is_verified && (
                                    <svg className="w-4 h-4 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                                      <path fillRule="evenodd" d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                                    </svg>
                                  )}
                                </div>
                                <p className="text-xs text-gray-500 dark:text-gray-400">@{user.username}</p>
                                {user.bio && (
                                  <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mt-1">
                                    {user.bio}
                                  </p>
                                )}
                                <div className="flex items-center space-x-4 mt-2 text-xs text-gray-400">
                                  <span>{user.followers_count} followers</span>
                                  <span>{user.articles_count} articles</span>
                                </div>
                              </div>
                            </div>
                          </a>
                        ))}
                      </div>
                    )}
                  </div>
                )}
              </div>

              {/* Search Tips */}
              {!query && (
                <div className="p-4 border-t dark:border-gray-700">
                  <div className="text-xs text-gray-500 space-y-1">
                    <p>• Search for articles, authors, or topics</p>
                    <p>• Use quotes for exact phrases</p>
                    <p>• Try "tag:technology" to search by tag</p>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
