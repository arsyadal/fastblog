import { useState } from 'react';
import { Eye, EyeOff, Loader2, Check, X } from 'lucide-react';

interface RegisterFormData {
  email: string;
  username: string;
  password: string;
  display_name: string;
}

interface ValidationErrors {
  email?: string;
  username?: string;
  password?: string;
  display_name?: string;
}

export default function RegisterForm() {
  const [formData, setFormData] = useState<RegisterFormData>({
    email: '',
    username: '',
    password: '',
    display_name: ''
  });
  const [showPassword, setShowPassword] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [validationErrors, setValidationErrors] = useState<ValidationErrors>({});

  // Real-time validation
  const validateField = (name: string, value: string): string | null => {
    switch (name) {
      case 'email':
        if (!value) return 'Email is required';
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) return 'Invalid email format';
        return null;
      case 'username':
        if (!value) return 'Username is required';
        if (value.length < 3) return 'Username must be at least 3 characters';
        if (value.length > 30) return 'Username cannot exceed 30 characters';
        if (!/^[a-zA-Z0-9_]+$/.test(value)) return 'Username can only contain letters, numbers, and underscores';
        return null;
      case 'password':
        if (!value) return 'Password is required';
        if (value.length < 8) return 'Password must be at least 8 characters';
        return null;
      case 'display_name':
        if (value && value.length > 100) return 'Display name cannot exceed 100 characters';
        return null;
      default:
        return null;
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError(null);

    // Validate all fields
    const errors: ValidationErrors = {};
    Object.entries(formData).forEach(([key, value]) => {
      const error = validateField(key, value);
      if (error) errors[key as keyof ValidationErrors] = error;
    });

    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors);
      setIsLoading(false);
      return;
    }

    try {
      const response = await fetch('/api/v1/auth/register', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData),
      });

      const data = await response.json();

      if (response.ok) {
        // Store token in localStorage
        localStorage.setItem('auth_token', data.token);
        localStorage.setItem('user', JSON.stringify(data.user));
        
        // Redirect to home page
        window.location.href = '/';
      } else {
        setError(data.message || 'Registration failed');
      }
    } catch (err) {
      setError('Network error. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));

    // Real-time validation
    const error = validateField(name, value);
    setValidationErrors(prev => ({
      ...prev,
      [name]: error
    }));
  };

  const getFieldValidationIcon = (fieldName: keyof RegisterFormData) => {
    const value = formData[fieldName];
    const error = validationErrors[fieldName];
    
    if (!value) return null;
    if (error) return <X className="h-5 w-5 text-red-500" />;
    return <Check className="h-5 w-5 text-green-500" />;
  };

  return (
    <div className="mt-8 space-y-6">
      <form className="space-y-6" onSubmit={handleSubmit}>
        {error && (
          <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-md p-4">
            <div className="text-sm text-red-700 dark:text-red-400">
              {error}
            </div>
          </div>
        )}

        <div>
          <label htmlFor="email" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            Email address
          </label>
          <div className="mt-1 relative">
            <input
              id="email"
              name="email"
              type="email"
              autoComplete="email"
              required
              value={formData.email}
              onChange={handleChange}
              className={`appearance-none relative block w-full px-3 py-2 pr-10 border ${
                validationErrors.email 
                  ? 'border-red-300 dark:border-red-600' 
                  : 'border-gray-300 dark:border-gray-600'
              } placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white bg-white dark:bg-gray-800 rounded-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm`}
              placeholder="Enter your email"
            />
            <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
              {getFieldValidationIcon('email')}
            </div>
          </div>
          {validationErrors.email && (
            <p className="mt-1 text-sm text-red-600 dark:text-red-400">{validationErrors.email}</p>
          )}
        </div>

        <div>
          <label htmlFor="username" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            Username
          </label>
          <div className="mt-1 relative">
            <input
              id="username"
              name="username"
              type="text"
              autoComplete="username"
              required
              value={formData.username}
              onChange={handleChange}
              className={`appearance-none relative block w-full px-3 py-2 pr-10 border ${
                validationErrors.username 
                  ? 'border-red-300 dark:border-red-600' 
                  : 'border-gray-300 dark:border-gray-600'
              } placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white bg-white dark:bg-gray-800 rounded-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm`}
              placeholder="Choose a username"
            />
            <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
              {getFieldValidationIcon('username')}
            </div>
          </div>
          {validationErrors.username && (
            <p className="mt-1 text-sm text-red-600 dark:text-red-400">{validationErrors.username}</p>
          )}
        </div>

        <div>
          <label htmlFor="display_name" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            Display name (optional)
          </label>
          <div className="mt-1 relative">
            <input
              id="display_name"
              name="display_name"
              type="text"
              value={formData.display_name}
              onChange={handleChange}
              className={`appearance-none relative block w-full px-3 py-2 pr-10 border ${
                validationErrors.display_name 
                  ? 'border-red-300 dark:border-red-600' 
                  : 'border-gray-300 dark:border-gray-600'
              } placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white bg-white dark:bg-gray-800 rounded-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm`}
              placeholder="Your full name"
            />
            <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
              {getFieldValidationIcon('display_name')}
            </div>
          </div>
          {validationErrors.display_name && (
            <p className="mt-1 text-sm text-red-600 dark:text-red-400">{validationErrors.display_name}</p>
          )}
        </div>

        <div>
          <label htmlFor="password" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            Password
          </label>
          <div className="mt-1 relative">
            <input
              id="password"
              name="password"
              type={showPassword ? 'text' : 'password'}
              autoComplete="new-password"
              required
              value={formData.password}
              onChange={handleChange}
              className={`appearance-none relative block w-full px-3 py-2 pr-20 border ${
                validationErrors.password 
                  ? 'border-red-300 dark:border-red-600' 
                  : 'border-gray-300 dark:border-gray-600'
              } placeholder-gray-500 dark:placeholder-gray-400 text-gray-900 dark:text-white bg-white dark:bg-gray-800 rounded-md focus:outline-none focus:ring-gray-500 focus:border-gray-500 focus:z-10 sm:text-sm`}
              placeholder="Create a password"
            />
            <div className="absolute inset-y-0 right-0 flex items-center">
              <div className="pr-3">
                {getFieldValidationIcon('password')}
              </div>
              <button
                type="button"
                className="pr-3 flex items-center"
                onClick={() => setShowPassword(!showPassword)}
              >
                {showPassword ? (
                  <EyeOff className="h-5 w-5 text-gray-400" />
                ) : (
                  <Eye className="h-5 w-5 text-gray-400" />
                )}
              </button>
            </div>
          </div>
          {validationErrors.password && (
            <p className="mt-1 text-sm text-red-600 dark:text-red-400">{validationErrors.password}</p>
          )}
          <div className="mt-2 text-xs text-gray-500 dark:text-gray-400">
            Password must be at least 8 characters long
          </div>
        </div>

        <div>
          <button
            type="submit"
            disabled={isLoading || Object.keys(validationErrors).some(key => validationErrors[key as keyof ValidationErrors])}
            className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-gray-900 hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 disabled:opacity-50 disabled:cursor-not-allowed dark:bg-white dark:text-gray-900 dark:hover:bg-gray-100"
          >
            {isLoading && (
              <Loader2 className="animate-spin -ml-1 mr-3 h-5 w-5" />
            )}
            {isLoading ? 'Creating account...' : 'Create account'}
          </button>
        </div>
      </form>

      {/* Terms */}
      <div className="text-center">
        <p className="text-xs text-gray-500 dark:text-gray-400">
          By creating an account, you agree to our{' '}
          <a href="/terms" className="text-gray-900 dark:text-white hover:underline">
            Terms of Service
          </a>{' '}
          and{' '}
          <a href="/privacy" className="text-gray-900 dark:text-white hover:underline">
            Privacy Policy
          </a>
        </p>
      </div>
    </div>
  );
}


