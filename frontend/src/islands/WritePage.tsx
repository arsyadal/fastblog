import { useState, useEffect, useRef } from 'react';
import ArticleEditor from './ArticleEditor.tsx';
import DraftSidebar from './DraftSidebar.tsx';
import type { DraftSidebarRef } from './DraftSidebar.tsx';

export default function WritePage() {
  const [currentDraftId, setCurrentDraftId] = useState<string | undefined>();
  const [articleData, setArticleData] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const draftSidebarRef = useRef<DraftSidebarRef>(null);

  // Get draft ID from URL if present
  useEffect(() => {
    const pathParts = window.location.pathname.split('/');
    if (pathParts.length === 3 && pathParts[1] === 'write') {
      const draftId = pathParts[2];
      setCurrentDraftId(draftId);
      loadDraft(draftId);
    }
  }, []);

  const loadDraft = async (draftId: string) => {
    try {
      setLoading(true);
      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      if (!token) {
        window.location.href = '/login?redirect=' + encodeURIComponent(window.location.pathname);
        return;
      }

      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      const response = await fetch(`${backendUrl}/api/v1/articles/drafts/${draftId}`, {
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      if (response.ok) {
        const data = await response.json();
        setArticleData({
          id: data.id,
          title: data.title || '',
          subtitle: data.subtitle || '',
          content: data.content || '',
          excerpt: data.excerpt || '',
          tags: data.tags || [],
          categories: data.categories || [],
          is_member_only: data.is_member_only || false,
        });
      } else if (response.status === 404) {
        // Draft not found, create new
        setArticleData(null);
        setCurrentDraftId(undefined);
        window.history.replaceState(null, '', '/write');
      } else {
        console.error('Failed to load draft:', response.status);
      }
    } catch (error) {
      console.error('Error loading draft:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDraftSelect = (draftId: string) => {
    setCurrentDraftId(draftId);
    window.history.pushState(null, '', `/write/${draftId}`);
    loadDraft(draftId);
  };

  // Callback when draft is saved - refresh sidebar
  const handleDraftSaved = () => {
    if (draftSidebarRef.current) {
      draftSidebarRef.current.refreshDrafts();
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 dark:border-white"></div>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen">
      {/* Header - responsive to sidebar state */}
      <header className="border-b border-gray-200 dark:border-gray-700 sticky top-0 z-30 bg-white dark:bg-gray-900">
        <div className={`transition-all duration-300 ${isSidebarOpen ? 'ml-80' : 'ml-0'}`}>
          <div className="max-w-4xl mx-auto px-4 py-4 flex items-center justify-center">
            <a
              href="/"
              className="text-2xl font-bold text-gray-900 dark:text-white hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
            >
              FastBlog
            </a>
          </div>
        </div>
      </header>

      {/* Main content area */}
      <div className="flex flex-1">
        <DraftSidebar
          ref={draftSidebarRef}
          onSelectDraft={handleDraftSelect}
          currentDraftId={currentDraftId}
          isOpen={isSidebarOpen}
          onToggle={(open) => setIsSidebarOpen(open)}
        />
        <div className={`flex-1 transition-all duration-300 ${isSidebarOpen ? 'ml-80' : 'ml-0'}`}>
          <div className="max-w-4xl mx-auto px-4 py-8">
            {/* Button to open sidebar when closed */}
            {!isSidebarOpen && (
              <button
                onClick={() => setIsSidebarOpen(true)}
                className="fixed left-4 top-20 z-50 flex items-center gap-2 px-4 py-2 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                title="Buka daftar draft"
              >
                <svg className="w-5 h-5 text-gray-600 dark:text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                </svg>
                <span className="text-sm font-medium text-gray-700 dark:text-gray-300">Drafts</span>
              </button>
            )}
            <ArticleEditor
              initialData={articleData || undefined}
              draftId={currentDraftId}
              onDraftSaved={handleDraftSaved}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

