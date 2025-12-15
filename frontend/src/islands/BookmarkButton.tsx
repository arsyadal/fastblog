import { useState, useEffect } from 'react';

interface BookmarkButtonProps {
    articleId: string;
    initialBookmarked?: boolean;
    className?: string;
    showText?: boolean;
}

export default function BookmarkButton({
    articleId,
    initialBookmarked = false,
    className = '',
    showText = false
}: BookmarkButtonProps) {
    const [isBookmarked, setIsBookmarked] = useState(initialBookmarked);
    const [isLoading, setIsLoading] = useState(false);
    const [isAuthenticated, setIsAuthenticated] = useState(false);

    useEffect(() => {
        const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
        setIsAuthenticated(!!token);

        // Check bookmark status if authenticated
        if (token) {
            checkBookmarkStatus(token);
        }
    }, [articleId]);

    const checkBookmarkStatus = async (token: string) => {
        try {
            const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
            const response = await fetch(`${backendUrl}/api/v1/articles/${articleId}`, {
                headers: {
                    'Authorization': `Bearer ${token}`,
                },
            });

            if (response.ok) {
                const data = await response.json();
                if (data.user_interactions) {
                    setIsBookmarked(data.user_interactions.has_bookmarked);
                }
            }
        } catch (error) {
            console.error('Error checking bookmark status:', error);
        }
    };

    const handleBookmark = async () => {
        const token = localStorage.getItem('token') || localStorage.getItem('auth_token');

        if (!token) {
            // Redirect to login
            window.location.href = '/login?redirect=' + encodeURIComponent(window.location.pathname);
            return;
        }

        setIsLoading(true);

        try {
            const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
            const method = isBookmarked ? 'DELETE' : 'POST';

            const response = await fetch(`${backendUrl}/api/v1/articles/${articleId}/bookmark`, {
                method,
                headers: {
                    'Authorization': `Bearer ${token}`,
                    'Content-Type': 'application/json',
                },
            });

            if (response.ok) {
                const data = await response.json();
                setIsBookmarked(data.bookmarked);
            } else if (response.status === 401) {
                // Token expired, redirect to login
                localStorage.removeItem('token');
                localStorage.removeItem('auth_token');
                window.location.href = '/login?redirect=' + encodeURIComponent(window.location.pathname);
            } else {
                console.error('Bookmark failed:', response.status);
            }
        } catch (error) {
            console.error('Error toggling bookmark:', error);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <button
            onClick={handleBookmark}
            disabled={isLoading}
            className={`flex items-center gap-2 transition-colors ${className} ${isBookmarked
                    ? 'text-yellow-600 dark:text-yellow-400'
                    : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
                } ${isLoading ? 'opacity-50 cursor-not-allowed' : ''}`}
            title={isBookmarked ? 'Remove from saved' : 'Save for later'}
        >
            {isBookmarked ? (
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
                </svg>
            ) : (
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
                </svg>
            )}
            {showText && (
                <span className="text-sm font-medium">
                    {isBookmarked ? 'Saved' : 'Save'}
                </span>
            )}
        </button>
    );
}
