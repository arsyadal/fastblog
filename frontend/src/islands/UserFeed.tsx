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
  share_url: string;
  share_title: string;
  share_description: string;
}

export default function UserFeed() {
  const [articles, setArticles] = useState<Article[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);

  useEffect(() => {
    fetchFeed();
  }, [page]);

  const fetchFeed = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      if (!token) {
        setError('Please login to view your feed');
        setLoading(false);
        return;
      }

      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      const response = await fetch(`${backendUrl}/api/v1/articles/feed?page=${page}&limit=20`, {
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        if (response.status === 401) {
          setError('Authentication expired. Please login again.');
          localStorage.removeItem('token');
          localStorage.removeItem('auth_token');
        } else {
          setError(`Failed to load feed (${response.status})`);
        }
        setLoading(false);
        return;
      }

      const data = await response.json();
      if (page === 1) {
        setArticles(data.articles || []);
      } else {
        setArticles(prev => [...prev, ...(data.articles || [])]);
      }
      setHasMore((data.articles || []).length === 20);
    } catch (err) {
      console.error('Failed to fetch feed:', err);
      setError('Failed to load feed. Please try again.');
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

  if (loading && articles.length === 0) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white"></div>
      </div>
    );
  }

  if (error && articles.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-red-600 dark:text-red-400 mb-4">{error}</p>
        <button
          onClick={() => window.location.href = '/login'}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
        >
          Login
        </button>
      </div>
    );
  }

  if (articles.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-600 dark:text-gray-400 mb-4">Your feed is empty</p>
        <p className="text-sm text-gray-500 dark:text-gray-500">
          Follow some users to see their articles in your feed
        </p>
      </div>
    );
  }

  return (
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
            </div>

            <ShareButtons
              url={article.share_url}
              title={article.share_title}
              description={article.share_description}
            />
          </div>
        </article>
      ))}

      {hasMore && (
        <div className="text-center py-6">
          <button
            onClick={() => setPage(prev => prev + 1)}
            disabled={loading}
            className="px-6 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 disabled:opacity-50"
          >
            {loading ? 'Loading...' : 'Load More'}
          </button>
        </div>
      )}
    </div>
  );
}
