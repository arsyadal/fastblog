import { useState, useEffect } from 'react';
import { Heart } from 'lucide-react';

interface ClapButtonProps {
  articleId: string;
  initialClaps: number;
  userClaps?: number;
  className?: string;
}

export default function ClapButton({ 
  articleId, 
  initialClaps, 
  userClaps = 0, 
  className = '' 
}: ClapButtonProps) {
  const [claps, setClaps] = useState(initialClaps);
  const [userClapCount, setUserClapCount] = useState(userClaps);
  const [isAnimating, setIsAnimating] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const handleClap = async () => {
    if (isLoading) return;

    const token = localStorage.getItem('token') || localStorage.getItem('auth_token');
    if (!token) {
      // Redirect to login
      window.location.href = '/login';
      return;
    }

    setIsLoading(true);
    setIsAnimating(true);

    try {
      const backendUrl = import.meta.env.BACKEND_URL || 'http://localhost:3001';
      const response = await fetch(`${backendUrl}/api/v1/articles/${articleId}/clap`, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ clap_count: 1 }),
      });

      if (response.ok) {
        const data = await response.json();
        setClaps(data.total_claps);
        setUserClapCount(data.user_claps);
      } else {
        if (response.status === 401) {
          window.location.href = '/login';
        } else {
          throw new Error('Failed to clap');
        }
      }
    } catch (error) {
      console.error('Clap failed:', error);
      // Revert optimistic update if needed
    } finally {
      setIsLoading(false);
      setTimeout(() => setIsAnimating(false), 300);
    }
  };

  return (
    <div className={`flex items-center space-x-2 ${className}`}>
      <button
        onClick={handleClap}
        disabled={isLoading}
        className={`
          relative flex items-center justify-center w-10 h-10 rounded-full border-2 transition-all duration-200
          ${userClapCount > 0 
            ? 'border-gray-900 bg-gray-100 text-gray-900 dark:border-gray-100 dark:bg-gray-800 dark:text-gray-100' 
            : 'border-gray-300 text-gray-600 hover:border-gray-900 hover:text-gray-900 dark:border-gray-600 dark:text-gray-400 dark:hover:border-gray-100 dark:hover:text-gray-100'
          }
          ${isAnimating ? 'scale-110' : 'scale-100'}
          ${isLoading ? 'opacity-50' : 'opacity-100'}
          disabled:cursor-not-allowed
        `}
        aria-label={userClapCount > 0 ? 'Unclap this article' : 'Clap for this article'}
      >
        <Heart 
          className={`h-5 w-5 transition-all duration-200 ${
            userClapCount > 0 ? 'fill-current' : ''
          } ${isAnimating ? 'scale-125' : 'scale-100'}`} 
        />
        
        {/* Clap animation particles */}
        {isAnimating && (
          <div className="absolute inset-0 pointer-events-none">
            {[...Array(3)].map((_, i) => (
              <div
                key={i}
                className="absolute w-2 h-2 bg-gray-900 dark:bg-gray-100 rounded-full animate-ping"
                style={{
                  top: `${20 + i * 10}%`,
                  left: `${30 + i * 15}%`,
                  animationDelay: `${i * 100}ms`,
                  animationDuration: '600ms',
                }}
              />
            ))}
          </div>
        )}
      </button>

      <div className="flex flex-col items-start">
        <span className="text-sm font-medium text-gray-900 dark:text-white">
          {claps.toLocaleString()}
        </span>
        {userClapCount > 0 && (
          <span className="text-xs text-gray-500 dark:text-gray-400">
            You clapped
          </span>
        )}
      </div>
    </div>
  );
}
