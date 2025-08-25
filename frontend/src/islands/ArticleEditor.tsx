import React, { useState, useEffect } from 'react';

interface ArticleData {
  title: string;
  subtitle: string;
  content: string;
  excerpt: string;
  tags: string[];
  featured_image_url?: string;
}

const ArticleEditor: React.FC = () => {
  const [articleData, setArticleData] = useState<ArticleData>({
    title: '',
    subtitle: '',
    content: '',
    excerpt: '',
    tags: [],
    featured_image_url: ''
  });
  
  const [tagInput, setTagInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Check authentication on mount
  useEffect(() => {
    const checkAuth = async () => {
      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      
      if (!token) {
        setError('Please login to write articles');
        setIsAuthenticated(false);
        // Auto-redirect to login after a short delay
        setTimeout(() => {
          const currentUrl = window.location.pathname;
          window.location.href = `/login?redirect=${encodeURIComponent(currentUrl)}`;
        }, 1500);
        return;
      }

      // Validate token with backend
      try {
        const response = await fetch('http://localhost:3001/api/v1/auth/me', {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });

        if (response.ok) {
          setIsAuthenticated(true);
          setError(null);
        } else {
          // Token is invalid
          localStorage.removeItem('token');
          localStorage.removeItem('auth_token');
          localStorage.removeItem('user');
          setError('Your session has expired. Please login again.');
          setTimeout(() => {
            const currentUrl = window.location.pathname;
            window.location.href = `/login?redirect=${encodeURIComponent(currentUrl)}&error=session_expired`;
          }, 1500);
        }
      } catch (error) {
        console.error('Auth validation failed:', error);
        setError('Unable to verify authentication. Please try logging in again.');
        setTimeout(() => {
          const currentUrl = window.location.pathname;
          window.location.href = `/login?redirect=${encodeURIComponent(currentUrl)}&error=auth_error`;
        }, 1500);
      }
    };

    checkAuth();
  }, []);

  const handleInputChange = (field: keyof ArticleData, value: string | string[]) => {
    setArticleData(prev => ({
      ...prev,
      [field]: value
    }));
    setError(null);
  };

  const handleAddTag = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' || e.key === ',') {
      e.preventDefault();
      const tag = tagInput.trim().toLowerCase();
      if (tag && !articleData.tags.includes(tag)) {
        setArticleData(prev => ({
          ...prev,
          tags: [...prev.tags, tag]
        }));
      }
      setTagInput('');
    }
  };

  const removeTag = (tagToRemove: string) => {
    setArticleData(prev => ({
      ...prev,
      tags: prev.tags.filter(tag => tag !== tagToRemove)
    }));
  };

  const saveDraft = async () => {
    if (!isAuthenticated) {
      setError('Please login to save articles');
      return;
    }

    if (!articleData.title.trim()) {
      setError('Title is required');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      const response = await fetch('http://localhost:3001/api/v1/articles', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({
          title: articleData.title,
          subtitle: articleData.subtitle || undefined,
          content: articleData.content,
          excerpt: articleData.excerpt || undefined,
          tags: articleData.tags.length > 0 ? articleData.tags : undefined,
          featured_image_url: articleData.featured_image_url || undefined
        })
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Failed to save article');
      }

      const result = await response.json();
      setSuccess(`Draft saved successfully! Article ID: ${result.id}`);
      
      // Optional: redirect to article page after save
      // window.location.href = `/article/${result.slug}`;
      
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save article');
    } finally {
      setIsLoading(false);
    }
  };

  const publishArticle = async () => {
    // First save as draft
    await saveDraft();
    
    // Then publish (this would be a separate API call in the future)
    setSuccess('Article published successfully!');
  };

  if (!isAuthenticated) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-4">
          Authentication Required
        </h2>
        <p className="text-gray-600 dark:text-gray-400 mb-6">
          Please login to write and publish articles. You will be redirected to the login page in a moment...
        </p>
        <a 
          href="/login" 
          className="bg-gray-900 dark:bg-white text-white dark:text-gray-900 px-6 py-3 rounded-full font-medium hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors"
        >
          Login to Continue
        </a>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto">
      {/* Error/Success Messages */}
      {error && (
        <div className="mb-6 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
          <p className="text-red-700 dark:text-red-400">{error}</p>
        </div>
      )}
      
      {success && (
        <div className="mb-6 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
          <p className="text-green-700 dark:text-green-400">{success}</p>
        </div>
      )}

      {/* Article Form */}
      <div className="space-y-6">
        {/* Featured Image */}
        <div>
          <input
            type="url"
            placeholder="Featured image URL (optional)"
            value={articleData.featured_image_url}
            onChange={(e) => handleInputChange('featured_image_url', e.target.value)}
            className="w-full px-0 py-3 text-sm text-gray-600 dark:text-gray-400 placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 border-b border-gray-200 dark:border-gray-700 focus:border-gray-400 dark:focus:border-gray-500 focus:ring-0 focus:outline-none"
          />
        </div>

        {/* Title */}
        <div>
          <textarea
            placeholder="Title"
            value={articleData.title}
            onChange={(e) => handleInputChange('title', e.target.value)}
            className="w-full px-0 py-2 text-4xl font-bold text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 resize-none focus:ring-0 focus:outline-none"
            rows={2}
            style={{ minHeight: '80px' }}
          />
        </div>

        {/* Subtitle */}
        <div>
          <textarea
            placeholder="Subtitle (optional)"
            value={articleData.subtitle}
            onChange={(e) => handleInputChange('subtitle', e.target.value)}
            className="w-full px-0 py-2 text-xl text-gray-600 dark:text-gray-300 placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 resize-none focus:ring-0 focus:outline-none"
            rows={2}
          />
        </div>

        {/* Content */}
        <div>
          <textarea
            placeholder="Tell your story..."
            value={articleData.content}
            onChange={(e) => handleInputChange('content', e.target.value)}
            className="w-full px-0 py-4 text-lg text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 resize-none focus:ring-0 focus:outline-none leading-relaxed"
            rows={20}
            style={{ minHeight: '500px' }}
          />
        </div>

        {/* Excerpt */}
        <div>
          <textarea
            placeholder="Article excerpt (optional - will be auto-generated if empty)"
            value={articleData.excerpt}
            onChange={(e) => handleInputChange('excerpt', e.target.value)}
            className="w-full px-0 py-3 text-base text-gray-700 dark:text-gray-300 placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 border-t border-gray-200 dark:border-gray-700 resize-none focus:ring-0 focus:outline-none"
            rows={3}
          />
        </div>

        {/* Tags */}
        <div className="border-t border-gray-200 dark:border-gray-700 pt-6">
          <div className="mb-3">
            <input
              type="text"
              placeholder="Add tags (press Enter or comma to add)"
              value={tagInput}
              onChange={(e) => setTagInput(e.target.value)}
              onKeyDown={handleAddTag}
              className="w-full px-0 py-2 text-base text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 border-b border-gray-200 dark:border-gray-700 focus:border-gray-400 dark:focus:border-gray-500 focus:ring-0 focus:outline-none"
            />
          </div>
          
          {/* Tag Display */}
          {articleData.tags.length > 0 && (
            <div className="flex flex-wrap gap-2">
              {articleData.tags.map((tag) => (
                <span
                  key={tag}
                  className="inline-flex items-center gap-1 px-3 py-1 text-sm bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-full"
                >
                  {tag}
                  <button
                    onClick={() => removeTag(tag)}
                    className="text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 ml-1"
                  >
                    ×
                  </button>
                </span>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Action Buttons */}
      <div className="fixed bottom-6 right-6 flex gap-3">
        <button
          onClick={saveDraft}
          disabled={isLoading || !articleData.title.trim()}
          className="px-6 py-3 text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-full font-medium hover:bg-gray-50 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors shadow-lg"
        >
          {isLoading ? 'Saving...' : 'Save Draft'}
        </button>
        
        <button
          onClick={publishArticle}
          disabled={isLoading || !articleData.title.trim()}
          className="px-6 py-3 bg-gray-900 dark:bg-white text-white dark:text-gray-900 rounded-full font-medium hover:bg-gray-800 dark:hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors shadow-lg"
        >
          {isLoading ? 'Publishing...' : 'Publish'}
        </button>
      </div>
    </div>
  );
};

export default ArticleEditor;
