import { useState, useEffect } from 'react';
import { User, LogOut, Settings, BookOpen, Edit3 } from 'lucide-react';

interface User {
  id: string;
  username: string;
  display_name: string | null;
  avatar_url: string | null;
  is_verified: boolean;
  email: string;
}

export default function AuthButtons() {
  const [user, setUser] = useState<User | null>(null);
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    // Check if user is logged in
    checkAuthStatus();
  }, []);

  const checkAuthStatus = async () => {
    try {
      const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
      if (!token) {
        setIsLoading(false);
        return;
      }

      const response = await fetch('/api/v1/auth/me', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });

      if (response.ok) {
        const userData = await response.json();
        setUser(userData);
      } else {
        localStorage.removeItem('token');
        localStorage.removeItem('auth_token');
        localStorage.removeItem('user');
      }
    } catch (error) {
      console.error('Auth check failed:', error);
      localStorage.removeItem('token');
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user');
    } finally {
      setIsLoading(false);
    }
  };

  const handleLogout = async () => {
    try {
      await fetch('/api/v1/auth/logout', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token') || localStorage.getItem('auth_token')}`
        }
      });
    } catch (error) {
      console.error('Logout failed:', error);
    } finally {
      localStorage.removeItem('token');
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user');
      setUser(null);
      setIsDropdownOpen(false);
      window.location.href = '/';
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center space-x-2">
        <div className="w-8 h-8 bg-gray-200 dark:bg-gray-700 rounded-full animate-pulse"></div>
      </div>
    );
  }

  if (!user) {
    return (
      <div className="flex items-center space-x-2">
        <a
          href="/login"
          className="px-4 py-2 text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition-colors"
        >
          Sign in
        </a>
        <a
          href="/register"
          className="px-4 py-2 bg-gray-900 text-white rounded-full hover:bg-gray-800 dark:bg-white dark:text-gray-900 dark:hover:bg-gray-100 transition-colors text-sm font-medium"
        >
          Get started
        </a>
      </div>
    );
  }

  return (
    <div className="relative flex items-center space-x-4">
      {/* Write Button */}
      <a
        href="/write"
        className="flex items-center space-x-1 px-3 py-2 text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition-colors"
      >
        <Edit3 className="h-4 w-4" />
        <span className="hidden sm:inline">Write</span>
      </a>

      {/* User Menu */}
      <div className="relative">
        <button
          onClick={() => setIsDropdownOpen(!isDropdownOpen)}
          className="flex items-center space-x-2 p-1 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
        >
          {user.avatar_url ? (
            <img
              src={user.avatar_url.startsWith('/') ? `http://localhost:3001${user.avatar_url}` : user.avatar_url}
              alt={user.display_name || user.username}
              className="w-8 h-8 rounded-full border-2 border-gray-200 dark:border-gray-700"
            />
          ) : (
            <div className="w-8 h-8 rounded-full border-2 border-gray-200 dark:border-gray-700 bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
              <User className="h-4 w-4 text-gray-600 dark:text-gray-400" />
            </div>
          )}
        </button>

        {/* Dropdown Menu */}
        {isDropdownOpen && (
          <div className="absolute right-0 mt-2 w-56 bg-white dark:bg-gray-800 border dark:border-gray-700 rounded-lg shadow-lg z-50">
            {/* User Info */}
            <div className="px-4 py-3 border-b dark:border-gray-700">
              <div className="flex items-center space-x-3">
                {user.avatar_url ? (
                  <img
                    src={user.avatar_url.startsWith('/') ? `http://localhost:3001${user.avatar_url}` : user.avatar_url}
                    alt={user.display_name || user.username}
                    className="w-10 h-10 rounded-full"
                  />
                ) : (
                  <div className="w-10 h-10 rounded-full bg-gray-100 dark:bg-gray-700 flex items-center justify-center">
                    <User className="h-5 w-5 text-gray-600 dark:text-gray-400" />
                  </div>
                )}
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium text-gray-900 dark:text-white truncate">
                    {user.display_name || user.username}
                  </p>
                  <p className="text-sm text-gray-500 dark:text-gray-400 truncate">
                    @{user.username}
                  </p>
                </div>
              </div>
            </div>

            {/* Menu Items */}
            <div className="py-1">
              <a
                href={`/@${user.username}`}
                className="flex items-center px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <User className="h-4 w-4 mr-3" />
                Profile
              </a>
              <a
                href="/library"
                className="flex items-center px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <BookOpen className="h-4 w-4 mr-3" />
                Library
              </a>
              <a
                href="/profile"
                className="flex items-center px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <Settings className="h-4 w-4 mr-3" />
                Profile Settings
              </a>
            </div>

            {/* Logout */}
            <div className="border-t dark:border-gray-700">
              <button
                onClick={handleLogout}
                className="flex items-center w-full px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <LogOut className="h-4 w-4 mr-3" />
                Sign out
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Click outside to close */}
      {isDropdownOpen && (
        <div
          className="fixed inset-0 z-40"
          onClick={() => setIsDropdownOpen(false)}
        />
      )}
    </div>
  );
}
