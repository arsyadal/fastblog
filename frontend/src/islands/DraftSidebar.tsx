import { useState, useEffect, forwardRef, useImperativeHandle } from 'react';

interface Draft {
  id: string;
  title: string;
  subtitle?: string;
  excerpt?: string;
  updated_at: string;
  created_at: string;
  last_auto_save?: string;
}

interface DraftSidebarProps {
  onSelectDraft: (draftId: string) => void;
  currentDraftId?: string;
  isOpen?: boolean;
  onToggle?: (open: boolean) => void;
}

export interface DraftSidebarRef {
  refreshDrafts: () => void;
}

const DraftSidebar = forwardRef<DraftSidebarRef, DraftSidebarProps>(({ onSelectDraft, currentDraftId, isOpen: controlledIsOpen, onToggle }, ref) => {
  const [drafts, setDrafts] = useState<Draft[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [internalIsOpen, setInternalIsOpen] = useState(true);

  // Use controlled state if provided, otherwise use internal state
  const isOpen = controlledIsOpen !== undefined ? controlledIsOpen : internalIsOpen;

  const setIsOpen = (open: boolean) => {
    if (onToggle) {
      onToggle(open);
    } else {
      setInternalIsOpen(open);
    }
  };

  useEffect(() => {
    fetchDrafts();
  }, []);

  const fetchDrafts = async () => {
    try {
      setLoading(true);
      setError(null);

      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      if (!token) {
        setError('Authentication required');
        setLoading(false);
        return;
      }

      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      const response = await fetch(`${backendUrl}/api/v1/articles/drafts`, {
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
          setError(`Failed to load drafts (${response.status})`);
        }
        setLoading(false);
        return;
      }

      const data = await response.json();
      setDrafts(data.drafts || []);
    } catch (err) {
      console.error('Failed to fetch drafts:', err);
      setError('Failed to load drafts. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  // Expose refreshDrafts method to parent component
  useImperativeHandle(ref, () => ({
    refreshDrafts: fetchDrafts
  }));

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

  const handleDraftClick = (draftId: string) => {
    onSelectDraft(draftId);
  };

  return (
    <div className={`fixed left-0 top-0 h-full w-80 bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-700 z-40 transition-transform duration-300 ${isOpen ? 'translate-x-0' : '-translate-x-full'
      }`}>
      {/* Header */}
      <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white">Drafts</h2>
          {!loading && (
            <span className="text-sm text-gray-500 dark:text-gray-400">
              ({drafts.length})
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={fetchDrafts}
            className="p-1.5 text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
            title="Refresh"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
          </button>
          <button
            onClick={() => setIsOpen(false)}
            className="p-1.5 text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
            title="Close sidebar"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      {/* Content */}
      <div className="h-[calc(100vh-73px)] overflow-y-auto">
        {loading ? (
          <div className="p-4 text-center text-gray-500 dark:text-gray-400">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 dark:border-white mx-auto"></div>
            <p className="mt-2 text-sm">Loading drafts...</p>
          </div>
        ) : error ? (
          <div className="p-4 text-center">
            <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
            <button
              onClick={fetchDrafts}
              className="mt-2 text-sm text-blue-600 dark:text-blue-400 hover:underline"
            >
              Try again
            </button>
          </div>
        ) : drafts.length === 0 ? (
          <div className="p-4 text-center text-gray-500 dark:text-gray-400">
            <svg className="w-12 h-12 mx-auto mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            <p className="text-sm">No drafts yet</p>
            <p className="text-xs mt-1">Start writing to create your first draft</p>
          </div>
        ) : (
          <div className="p-2">
            {drafts.map((draft) => (
              <button
                key={draft.id}
                onClick={() => handleDraftClick(draft.id)}
                className={`w-full text-left p-3 rounded-lg mb-2 transition-colors ${currentDraftId === draft.id
                  ? 'bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600'
                  : 'hover:bg-gray-50 dark:hover:bg-gray-800'
                  }`}
              >
                <h3 className="font-medium text-gray-900 dark:text-white truncate mb-1">
                  {draft.title || 'Untitled'}
                </h3>
                {draft.subtitle && (
                  <p className="text-sm text-gray-600 dark:text-gray-400 truncate mb-1">
                    {draft.subtitle}
                  </p>
                )}
                {draft.excerpt && (
                  <p className="text-xs text-gray-500 dark:text-gray-500 line-clamp-2 mb-2">
                    {draft.excerpt}
                  </p>
                )}
                <div className="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
                  <span>{formatDate(draft.updated_at)}</span>
                  {draft.last_auto_save && (
                    <>
                      <span>•</span>
                      <span className="text-green-600 dark:text-green-400">Auto-saved</span>
                    </>
                  )}
                </div>
              </button>
            ))}
          </div>
        )}
      </div>

    </div>
  );
});

DraftSidebar.displayName = 'DraftSidebar';

export default DraftSidebar;
