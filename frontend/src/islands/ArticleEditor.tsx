import { useState, useEffect, useCallback } from 'react';

interface ArticleEditorProps {
  initialData?: {
    id?: string;
    title: string;
    subtitle?: string;
    content: string;
    excerpt?: string;
    tags?: string[];
    categories?: string[];
    is_member_only?: boolean;
  };
  onSave?: (data: any) => void;
  onPublish?: (data: any) => void;
  draftId?: string;
  onDraftSelect?: (draftId: string) => void;
  onDraftSaved?: () => void;
}

interface AutoSaveState {
  isAutoSaving: boolean;
  lastSaved: Date | null;
  version: number;
}

interface NotificationState {
  show: boolean;
  message: string;
  type: 'success' | 'error' | 'info';
}

export default function ArticleEditor({ initialData, onSave, onPublish, onDraftSaved }: ArticleEditorProps) {
  const [formData, setFormData] = useState({
    title: initialData?.title || '',
    subtitle: initialData?.subtitle || '',
    content: initialData?.content || '',
    excerpt: initialData?.excerpt || '',
    tags: initialData?.tags || [],
    categories: initialData?.categories || [],
    is_member_only: initialData?.is_member_only || false,
  });

  const [autoSaveState, setAutoSaveState] = useState<AutoSaveState>({
    isAutoSaving: false,
    lastSaved: null,
    version: 0,
  });

  const [notification, setNotification] = useState<NotificationState>({
    show: false,
    message: '',
    type: 'info',
  });

  const [isSaving, setIsSaving] = useState(false);
  const [newTag, setNewTag] = useState('');
  const [newCategory, setNewCategory] = useState('');
  const [availableCategories] = useState([
    'Technology', 'Business', 'Health', 'Science', 'Politics',
    'Entertainment', 'Sports', 'Education', 'Travel', 'Food'
  ]);

  // Check authentication on mount
  useEffect(() => {
    const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
    if (!token) {
      // Redirect to login if not authenticated
      window.location.href = '/login?redirect=' + encodeURIComponent(window.location.pathname);
    }
  }, []);

  // Show notification helper
  const showNotification = (message: string, type: 'success' | 'error' | 'info' = 'info') => {
    setNotification({ show: true, message, type });
    setTimeout(() => {
      setNotification({ show: false, message: '', type: 'info' });
    }, 4000);
  };

  // Auto-save functionality
  const autoSave = useCallback(async () => {
    if (!formData.content.trim()) return;

    const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
    if (!token) {
      // Don't auto-save if not logged in, but don't show error (user might be typing)
      return;
    }

    setAutoSaveState(prev => ({ ...prev, isAutoSaving: true }));

    try {

      // Use backend URL instead of frontend
      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      const response = await fetch(`${backendUrl}/api/v1/articles/draft/auto-save`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({
          article_id: initialData?.id || null,
          ...formData,
        }),
      });

      if (response.ok) {
        const result = await response.json();
        setAutoSaveState(prev => ({
          isAutoSaving: false,
          lastSaved: new Date(),
          version: prev.version + 1,
        }));

        // Update URL if this is a new article
        if (!initialData?.id && result.article_id) {
          window.history.replaceState(null, '', `/write/${result.article_id}`);
        }
      } else {
        // Handle different HTTP status codes
        let errorMessage = 'Auto-save failed';
        if (response.status === 401) {
          errorMessage = 'Authentication expired. Please login again.';
          // Clear invalid token
          localStorage.removeItem('token');
          localStorage.removeItem('auth_token');
          // Redirect to login after showing error
          setTimeout(() => {
            window.location.href = '/login?redirect=' + encodeURIComponent(window.location.pathname);
          }, 2000);
        } else if (response.status === 403) {
          errorMessage = 'You do not have permission to save articles.';
        } else if (response.status === 404) {
          errorMessage = 'Auto-save endpoint not found. Please contact support.';
        } else if (response.status === 500) {
          errorMessage = 'Server error. Please try again later.';
        }
        console.error(`Auto-save failed with status ${response.status}:`, errorMessage);
        setAutoSaveState(prev => ({ ...prev, isAutoSaving: false }));
      }
    } catch (error) {
      console.error('Auto-save failed:', error);
      // Show user-friendly error message
      if (error instanceof TypeError && error.message.includes('fetch')) {
        console.error('Network error - backend might be down or CORS issue');
      }
    } finally {
      setAutoSaveState(prev => ({ ...prev, isAutoSaving: false }));
    }
  }, [formData, initialData?.id]);

  // Auto-save on content change (debounced)
  useEffect(() => {
    const timer = setTimeout(() => {
      autoSave();
    }, 3000); // Auto-save after 3 seconds of inactivity

    return () => clearTimeout(timer);
  }, [formData.content, autoSave]);

  const handleInputChange = (field: string, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  const addTag = () => {
    if (newTag.trim() && !formData.tags.includes(newTag.trim())) {
      setFormData(prev => ({
        ...prev,
        tags: [...prev.tags, newTag.trim()]
      }));
      setNewTag('');
    }
  };

  const removeTag = (tagToRemove: string) => {
    setFormData(prev => ({
      ...prev,
      tags: prev.tags.filter(tag => tag !== tagToRemove)
    }));
  };

  const addCategory = () => {
    if (newCategory.trim() && !formData.categories.includes(newCategory.trim())) {
      setFormData(prev => ({
        ...prev,
        categories: [...prev.categories, newCategory.trim()]
      }));
      setNewCategory('');
    }
  };

  const removeCategory = (categoryToRemove: string) => {
    setFormData(prev => ({
      ...prev,
      categories: prev.categories.filter(cat => cat !== categoryToRemove)
    }));
  };

  const handleSave = async () => {
    if (!formData.content.trim()) {
      showNotification('Please add some content before saving', 'error');
      return;
    }

    setIsSaving(true);

    try {
      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      if (!token) {
        showNotification('Authentication required. Please login again.', 'error');
        return;
      }

      // Use backend URL instead of frontend
      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      const response = await fetch(`${backendUrl}/api/v1/articles/draft/auto-save`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({
          article_id: initialData?.id || null,
          ...formData,
        }),
      });

      if (response.ok) {
        const result = await response.json();

        // Update auto-save state
        setAutoSaveState(prev => ({
          isAutoSaving: false,
          lastSaved: new Date(),
          version: prev.version + 1,
        }));

        // Show success message
        if (initialData?.id) {
          showNotification('Draft updated successfully!', 'success');
        } else {
          showNotification('Draft created successfully!', 'success');
          // Update URL if this is a new article
          if (result.article_id) {
            window.history.replaceState(null, '', `/write/${result.article_id}`);
          }
        }

        // Call onSave callback if provided
        onSave?.(formData);

        // Notify parent to refresh drafts sidebar
        onDraftSaved?.();
      } else {
        // Handle different HTTP status codes with user-friendly messages
        let errorMessage = 'Failed to save draft';
        let errorType: 'error' | 'info' = 'error';

        if (response.status === 401) {
          errorMessage = 'Your session has expired. Please login again.';
          errorType = 'error';
        } else if (response.status === 403) {
          errorMessage = 'You do not have permission to save articles.';
          errorType = 'error';
        } else if (response.status === 404) {
          errorMessage = 'Save endpoint not found. Please contact support.';
          errorType = 'error';
        } else if (response.status === 422) {
          errorMessage = 'Invalid article data. Please check your input.';
          errorType = 'error';
        } else if (response.status === 500) {
          errorMessage = 'Server error. Please try again later.';
          errorType = 'error';
        } else if (response.status === 503) {
          errorMessage = 'Service temporarily unavailable. Please try again later.';
          errorType = 'info';
        }

        try {
          const errorData = await response.json();
          if (errorData.error) {
            errorMessage = `${errorMessage}: ${errorData.error}`;
          }
        } catch {
          // If we can't parse error response, use our default message
        }

        showNotification(errorMessage, errorType);
        console.error(`Save failed with status ${response.status}:`, errorMessage);
      }
    } catch (error) {
      console.error('Save failed:', error);

      // Provide specific error messages based on error type
      let errorMessage = 'Failed to save draft. Please try again.';

      if (error instanceof TypeError) {
        if (error.message.includes('fetch')) {
          errorMessage = 'Cannot connect to server. Please check your internet connection or try again later.';
        } else if (error.message.includes('Failed to fetch')) {
          errorMessage = 'Network error. Please check your connection and try again.';
        }
      } else if (error instanceof Error) {
        if (error.message.includes('CORS')) {
          errorMessage = 'Cross-origin request blocked. Please contact support.';
        }
      }

      showNotification(errorMessage, 'error');
    } finally {
      setIsSaving(false);
    }
  };

  const handlePublish = async () => {
    if (!formData.content.trim()) {
      showNotification('Please add some content before publishing', 'error');
      return;
    }

    if (!formData.title.trim()) {
      showNotification('Please add a title before publishing', 'error');
      return;
    }

    setIsSaving(true);

    try {
      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      if (!token) {
        showNotification('Authentication required. Please login again.', 'error');
        setTimeout(() => {
          window.location.href = '/login?redirect=' + encodeURIComponent(window.location.pathname);
        }, 2000);
        return;
      }

      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      let articleId = initialData?.id;

      // If article doesn't exist yet, create it first
      if (!articleId) {
        const createResponse = await fetch(`${backendUrl}/api/v1/articles`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`,
          },
          body: JSON.stringify({
            title: formData.title,
            subtitle: formData.subtitle || null,
            content: formData.content,
            excerpt: formData.excerpt || null,
            tags: formData.tags,
            categories: formData.categories,
            is_member_only: formData.is_member_only,
          }),
        });

        if (!createResponse.ok) {
          const errorData = await createResponse.json().catch(() => ({}));
          const errorMessage = errorData.error || `Failed to create article (${createResponse.status})`;
          showNotification(errorMessage, 'error');
          setIsSaving(false);
          return;
        }

        const createdArticle = await createResponse.json();
        articleId = createdArticle.id || createdArticle.article_id;

        // Update URL with new article ID
        if (articleId) {
          window.history.replaceState(null, '', `/write/${articleId}`);
        }
      } else {
        // Update existing article first
        const updateResponse = await fetch(`${backendUrl}/api/v1/articles/${articleId}`, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${token}`,
          },
          body: JSON.stringify({
            title: formData.title,
            subtitle: formData.subtitle || null,
            content: formData.content,
            excerpt: formData.excerpt || null,
            tags: formData.tags,
            categories: formData.categories,
            is_member_only: formData.is_member_only,
          }),
        });

        if (!updateResponse.ok) {
          const errorData = await updateResponse.json().catch(() => ({}));
          const errorMessage = errorData.error || `Failed to update article (${updateResponse.status})`;
          showNotification(errorMessage, 'error');
          setIsSaving(false);
          return;
        }
      }

      // Now publish the article
      const publishResponse = await fetch(`${backendUrl}/api/v1/articles/${articleId}/publish`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
      });

      if (publishResponse.ok) {
        const publishedArticle = await publishResponse.json();
        showNotification('Article published successfully!', 'success');

        // Call onPublish callback if provided
        onPublish?.(formData);

        // Redirect to published article after a short delay
        setTimeout(() => {
          const slug = publishedArticle.slug;
          if (slug) {
            window.location.href = `/article/${slug}`;
          } else {
            // Fallback: redirect to home if slug not available
            window.location.href = '/';
          }
        }, 1500);
      } else {
        const errorData = await publishResponse.json().catch(() => ({}));
        let errorMessage = errorData.error || `Failed to publish article (${publishResponse.status})`;

        if (publishResponse.status === 401) {
          errorMessage = 'Authentication expired. Please login again.';
          localStorage.removeItem('token');
          localStorage.removeItem('auth_token');
          setTimeout(() => {
            window.location.href = '/login?redirect=' + encodeURIComponent(window.location.pathname);
          }, 2000);
        } else if (publishResponse.status === 403) {
          errorMessage = 'You do not have permission to publish articles.';
        } else if (publishResponse.status === 404) {
          errorMessage = 'Article not found.';
        }

        showNotification(errorMessage, 'error');
      }
    } catch (error) {
      console.error('Publish failed:', error);
      let errorMessage = 'Failed to publish article. Please try again.';

      if (error instanceof TypeError && error.message.includes('fetch')) {
        errorMessage = 'Network error. Please check your connection and try again.';
      }

      showNotification(errorMessage, 'error');
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      {/* Notification Toast */}
      {notification.show && (
        <div className={`fixed top-4 right-4 z-50 px-6 py-3 rounded-lg shadow-lg transition-all duration-300 ${notification.type === 'success'
          ? 'bg-green-500 text-white'
          : notification.type === 'error'
            ? 'bg-red-500 text-white'
            : 'bg-blue-500 text-white'
          }`}>
          <div className="flex items-center space-x-2">
            {notification.type === 'success' && (
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
              </svg>
            )}
            {notification.type === 'error' && (
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
              </svg>
            )}
            {notification.type === 'info' && (
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
              </svg>
            )}
            <span className="font-medium">{notification.message}</span>
          </div>
        </div>
      )}

      {/* Auto-save indicator */}
      <div className="mb-6 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            {autoSaveState.isAutoSaving ? (
              <>
                <div className="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                <span className="text-sm text-gray-600 dark:text-gray-300">Auto-saving...</span>
              </>
            ) : (
              <>
                <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                <span className="text-sm text-gray-600 dark:text-gray-300">
                  {autoSaveState.lastSaved
                    ? `Last saved ${autoSaveState.lastSaved.toLocaleTimeString()}`
                    : 'Not saved yet'
                  }
                </span>
              </>
            )}
          </div>
          <span className="text-xs text-gray-500">v{autoSaveState.version}</span>
        </div>
      </div>

      {/* Title */}
      <div className="mb-6">
        <textarea
          placeholder="Title"
          value={formData.title}
          onChange={(e) => handleInputChange('title', e.target.value)}
          className="w-full px-0 py-2 text-4xl font-bold text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 resize-none focus:ring-0 focus:outline-none"
          rows={2}
          style={{ minHeight: '80px' }}
        />
      </div>

      {/* Subtitle */}
      <div className="mb-6">
        <textarea
          placeholder="Subtitle (optional)"
          value={formData.subtitle}
          onChange={(e) => handleInputChange('subtitle', e.target.value)}
          className="w-full px-0 py-2 text-xl text-gray-600 dark:text-gray-300 placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 resize-none focus:ring-0 focus:outline-none"
          rows={2}
        />
      </div>

      {/* Categories */}
      <div className="mb-6 border-t border-gray-200 dark:border-gray-700 pt-6">
        <div className="mb-3">
          <input
            type="text"
            placeholder="Add category..."
            value={newCategory}
            onChange={(e) => setNewCategory(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && addCategory()}
            className="w-full px-0 py-2 text-base text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 border-b border-gray-200 dark:border-gray-700 focus:border-gray-400 dark:focus:border-gray-500 focus:ring-0 focus:outline-none"
          />
        </div>

        {/* Category Display */}
        {formData.categories.length > 0 && (
          <div className="flex flex-wrap gap-2 mb-2">
            {formData.categories.map((category) => (
              <span
                key={category}
                className="inline-flex items-center gap-1 px-3 py-1 text-sm bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded-full"
              >
                {category}
                <button
                  onClick={() => removeCategory(category)}
                  className="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-200 ml-1"
                >
                  ×
                </button>
              </span>
            ))}
          </div>
        )}

        <div className="text-sm text-gray-500">
          Popular: {availableCategories.slice(0, 5).join(', ')}
        </div>
      </div>

      {/* Tags */}
      <div className="mb-6 border-t border-gray-200 dark:border-gray-700 pt-6">
        <div className="mb-3">
          <input
            type="text"
            placeholder="Add tags (press Enter or comma to add)"
            value={newTag}
            onChange={(e) => setNewTag(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter' || e.key === ',') {
                e.preventDefault();
                addTag();
              }
            }}
            className="w-full px-0 py-2 text-base text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 border-b border-gray-200 dark:border-gray-700 focus:border-gray-400 dark:focus:border-gray-500 focus:ring-0 focus:outline-none"
          />
        </div>

        {/* Tag Display */}
        {formData.tags.length > 0 && (
          <div className="flex flex-wrap gap-2">
            {formData.tags.map((tag) => (
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

      {/* Content */}
      <div className="mb-6">
        <textarea
          placeholder="Tell your story..."
          value={formData.content}
          onChange={(e) => handleInputChange('content', e.target.value)}
          className="w-full px-0 py-4 text-lg text-gray-900 dark:text-white placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 resize-none focus:ring-0 focus:outline-none leading-relaxed"
          rows={20}
          style={{ minHeight: '500px' }}
        />
      </div>

      {/* Excerpt */}
      <div className="mb-6 border-t border-gray-200 dark:border-gray-700 pt-6">
        <textarea
          placeholder="Article excerpt (optional - will be auto-generated if empty)"
          value={formData.excerpt}
          onChange={(e) => handleInputChange('excerpt', e.target.value)}
          className="w-full px-0 py-3 text-base text-gray-700 dark:text-gray-300 placeholder-gray-400 dark:placeholder-gray-500 bg-transparent border-0 border-t border-gray-200 dark:border-gray-700 resize-none focus:ring-0 focus:outline-none"
          rows={3}
        />
      </div>

      {/* Settings */}
      <div className="mb-6 border-t border-gray-200 dark:border-gray-700 pt-6">
        <label className="flex items-center">
          <input
            type="checkbox"
            checked={formData.is_member_only}
            onChange={(e) => handleInputChange('is_member_only', e.target.checked)}
            className="mr-2 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
          />
          <span className="text-sm text-gray-700 dark:text-gray-300">Member only</span>
        </label>
      </div>

      {/* Action Buttons */}
      <div className="fixed bottom-6 right-6 flex gap-3">
        <button
          onClick={handleSave}
          disabled={isSaving}
          className={`px-6 py-3 text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-full font-medium transition-colors shadow-lg ${isSaving
            ? 'opacity-50 cursor-not-allowed'
            : 'hover:bg-gray-50 dark:hover:bg-gray-700'
            }`}
        >
          {isSaving ? (
            <div className="flex items-center space-x-2">
              <div className="w-4 h-4 border-2 border-gray-400 border-t-transparent rounded-full animate-spin"></div>
              <span>Saving...</span>
            </div>
          ) : (
            'Save Draft'
          )}
        </button>

        <button
          onClick={handlePublish}
          disabled={isSaving}
          className={`px-6 py-3 bg-gray-900 dark:bg-white text-white dark:text-gray-900 rounded-full font-medium hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors shadow-lg ${isSaving ? 'opacity-50 cursor-not-allowed' : ''
            }`}
        >
          {isSaving ? (
            <span className="flex items-center gap-2">
              <svg className="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Publishing...
            </span>
          ) : (
            'Publish'
          )}
        </button>
      </div>
    </div>
  );
}
