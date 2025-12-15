import { useState, useEffect } from 'react';

interface ProfileActionButtonProps {
    profileUserId: string;
    profileUsername: string;
}

interface CurrentUser {
    id: string;
    username: string;
}

export default function ProfileActionButton({ profileUserId, profileUsername }: ProfileActionButtonProps) {
    const [currentUser, setCurrentUser] = useState<CurrentUser | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [isFollowing, setIsFollowing] = useState(false);
    const [isFollowLoading, setIsFollowLoading] = useState(false);

    useEffect(() => {
        checkCurrentUser();
    }, []);

    const checkCurrentUser = async () => {
        try {
            const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
            if (!token) {
                setIsLoading(false);
                return;
            }

            const backendUrl = import.meta.env.PUBLIC_BACKEND_URL || 'http://localhost:3001';
            const response = await fetch(`${backendUrl}/api/v1/auth/me`, {
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            });

            if (response.ok) {
                const userData = await response.json();
                setCurrentUser(userData);

                // Check if already following this user
                if (userData.id !== profileUserId) {
                    checkFollowingStatus(userData.id);
                }
            }
        } catch (error) {
            console.error('Auth check failed:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const checkFollowingStatus = async (currentUserId: string) => {
        try {
            const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
            if (!token) return;

            const backendUrl = import.meta.env.PUBLIC_BACKEND_URL || 'http://localhost:3001';
            const response = await fetch(`${backendUrl}/api/v1/users/${profileUserId}/follow-status`, {
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            });

            if (response.ok) {
                const data = await response.json();
                setIsFollowing(data.is_following);
            }
        } catch (error) {
            console.error('Failed to check follow status:', error);
        }
    };

    const handleFollow = async () => {
        const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
        if (!token) {
            window.location.href = '/login';
            return;
        }

        setIsFollowLoading(true);
        try {
            const backendUrl = import.meta.env.PUBLIC_BACKEND_URL || 'http://localhost:3001';
            const method = isFollowing ? 'DELETE' : 'POST';
            const response = await fetch(`${backendUrl}/api/v1/users/${profileUserId}/follow`, {
                method,
                headers: {
                    'Authorization': `Bearer ${token}`
                }
            });

            if (response.ok) {
                setIsFollowing(!isFollowing);
                // Reload to update follower count
                window.location.reload();
            }
        } catch (error) {
            console.error('Follow action failed:', error);
        } finally {
            setIsFollowLoading(false);
        }
    };

    if (isLoading) {
        return (
            <div className="mb-8">
                <button
                    disabled
                    className="bg-gray-300 dark:bg-gray-600 text-gray-500 dark:text-gray-400 px-8 py-3 rounded-full font-medium cursor-not-allowed"
                >
                    Loading...
                </button>
            </div>
        );
    }

    // If viewing own profile, show Edit Profile button
    if (currentUser && currentUser.id === profileUserId) {
        return (
            <div className="mb-8">
                <a
                    href="/profile"
                    className="inline-block bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white px-8 py-3 rounded-full font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
                >
                    <span className="flex items-center gap-2">
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                        Edit Profile
                    </span>
                </a>
            </div>
        );
    }

    // If not logged in, show Follow button that redirects to login
    if (!currentUser) {
        return (
            <div className="mb-8">
                <a
                    href={`/login?redirect=/@${profileUsername}`}
                    className="inline-block bg-gray-900 dark:bg-white text-white dark:text-gray-900 px-8 py-3 rounded-full font-medium hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors"
                >
                    Follow
                </a>
            </div>
        );
    }

    // If viewing another user's profile, show Follow/Unfollow button
    return (
        <div className="mb-8">
            <button
                onClick={handleFollow}
                disabled={isFollowLoading}
                className={`px-8 py-3 rounded-full font-medium transition-colors ${isFollowing
                        ? 'bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-red-100 dark:hover:bg-red-900/30 hover:text-red-600 dark:hover:text-red-400 border border-gray-300 dark:border-gray-600'
                        : 'bg-gray-900 dark:bg-white text-white dark:text-gray-900 hover:bg-gray-800 dark:hover:bg-gray-100'
                    } ${isFollowLoading ? 'opacity-50 cursor-not-allowed' : ''}`}
            >
                {isFollowLoading ? (
                    <span className="flex items-center gap-2">
                        <svg className="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        Processing...
                    </span>
                ) : isFollowing ? (
                    'Following'
                ) : (
                    'Follow'
                )}
            </button>
        </div>
    );
}
