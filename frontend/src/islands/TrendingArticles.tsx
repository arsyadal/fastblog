import { useState, useEffect } from 'react';
import ShareButtons from './ShareButtons';

interface Article {
  id: string;
  title: string;
  subtitle?: string;
  excerpt?: string;
  author: {
    username: string;
    display_name?: string;
    avatar_url?: string;
  };
  published_at?: string;
  reading_time_minutes: number;
  claps_count: number;
  comments_count: number;
  views_count: number;
  share_url: string;
  share_title: string;
  share_description: string;
}

export default function TrendingArticles() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [timeWindow, setTimeWindow] = useState(168); // 7 days

  useEffect(() => {
    fetchTrending();
  }, [timeWindow]);

  const fetchTrending = async () => {
    try {
      setLoading(true);
      setError(null);

      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      
      const headers: HeadersInit = {};
      if (token) {
        headers['Authorization'] = `Bearer ${token}`;
      }

      const response = await fetch(
        `${backendUrl}/api/v1/articles/trending?limit=20&time_window=${timeWindow}`,
        { headers }
      );

      if (!response.ok) {
        setError(`Failed to load trending articles (${response.status})`);
        setLoading(false);
        return;
      }

      const data = await response.json();
      setArticles(data.articles || []);
    } catch (err) {
      console.error('Failed to fetch trending articles:', err);
      setError('Failed to load trending articles. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-12">
        <p className="text-red-600 dark:text-red-400 mb-4">{error}</p>
        <button
          onClick={fetchTrending}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
        >
          Try Again
        </button>
      </div>
    );
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Trending Articles</h2>
        <select
          value={timeWindow}
          onChange={(e) => setTimeWindow(Number(e.target.value))}
          className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
        >
          <option value={24}>Last 24 hours</option>
          <option value={168}>Last 7 days</option>
          <option value={720}>Last 30 days</option>
        </select>
      </div>

      <div className="space-y-6">
        {articles.map((article) => (
          <article
            key={article.id}
            className="border-b border-gray-200 dark:border-gray-700 pb-6 last:border-0"
          >
            <div className="flex items-center gap-3 mb-3">
              {article.author.avatar_url ? (
                <img
                  src={article.author.avatar_url}
                  alt={article.author.display_name || article.author.username}
                  className="w-8 h-8 rounded-full"
                />
              ) : (
                <div className="w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center">
                  <span className="text-sm text-gray-600 dark:text-gray-400">
                    {(article.author.display_name || article.author.username)[0].toUpperCase()}
                  </span>
                </div>
              )}
              <div>
                <p className="text-sm font-medium text-gray-900 dark:text-white">
                  {article.author.display_name || article.author.username}
                </p>
                {article.published_at && (
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    {formatDate(article.published_at)}
                  </p>
                )}
              </div>
            </div>

            <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-2 hover:text-blue-600 dark:hover:text-blue-400">
              <a href={`/article/${article.id}`}>{article.title}</a>
            </h2>

            {article.subtitle && (
              <p className="text-lg text-gray-600 dark:text-gray-400 mb-2">{article.subtitle}</p>
            )}

            {article.excerpt && (
              <p className="text-gray-700 dark:text-gray-300 mb-4">{article.excerpt}</p>
            )}

            <div className="flex items-center justify-between">
              <div className="flex items-center gap-4 text-sm text-gray-500 dark:text-gray-400">
                <span>{article.reading_time_minutes} min read</span>
                <span>{article.claps_count} claps</span>
                <span>{article.comments_count} comments</span>
                <span>{article.views_count} views</span>
              </div>

              <ShareButtons
                url={article.share_url}
                title={article.share_title}
                description={article.share_description}
              />
            </div>
          </article>
        ))}

        {articles.length === 0 && (
          <div className="text-center py-12">
            <p className="text-gray-600 dark:text-gray-400">No trending articles found</p>
          </div>
        )}
      </div>
    </div>
  );
}
