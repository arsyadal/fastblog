'use client'

import { useState } from 'react'

interface NewsletterProps {
  variant?: 'primary' | 'secondary'
  title?: string
  description?: string
}

export default function Newsletter({ 
  variant = 'primary', 
  title = 'Stay Updated',
  description = 'Get the latest articles and insights delivered to your inbox'
}: NewsletterProps) {
  const [email, setEmail] = useState('')
  const [isSubscribed, setIsSubscribed] = useState(false)
  const [isLoading, setIsLoading] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!email) return

    setIsLoading(true)
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    setIsSubscribed(true)
    setIsLoading(false)
    setEmail('')
  }

  const isPrimary = variant === 'primary'
  const bgColor = isPrimary ? 'bg-primary-600' : 'bg-gray-100'
  const textColor = isPrimary ? 'text-white' : 'text-gray-900'
  const accentColor = isPrimary ? 'text-primary-100' : 'text-gray-600'
  const buttonBg = isPrimary ? 'bg-white text-primary-700 hover:bg-gray-100' : 'bg-primary-600 text-white hover:bg-primary-700'
  const inputBg = isPrimary ? 'bg-white' : 'bg-white'

  if (isSubscribed) {
    return (
      <section className={`${bgColor} py-16`}>
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <div className="text-6xl mb-4">🎉</div>
          <h2 className={`text-3xl font-bold ${textColor} mb-4`}>
            Welcome to FastBlog!
          </h2>
          <p className={`text-xl ${accentColor} mb-8`}>
            You've successfully subscribed to our newsletter. Check your email for confirmation.
          </p>
          <button
            onClick={() => setIsSubscribed(false)}
            className="btn bg-white text-primary-700 hover:bg-gray-100 px-6 py-3 font-semibold"
          >
            Subscribe Another Email
          </button>
        </div>
      </section>
    )
  }

  return (
    <section className={`${bgColor} py-16`}>
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h2 className={`text-3xl font-bold ${textColor} mb-4`}>
          {title}
        </h2>
        <p className={`text-xl ${accentColor} mb-8`}>
          {description}
        </p>
        <form onSubmit={handleSubmit} className="flex flex-col sm:flex-row gap-4 max-w-md mx-auto">
          <input
            type="email"
            placeholder="Enter your email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            className={`input flex-1 ${inputBg}`}
            required
          />
          <button
            type="submit"
            disabled={isLoading}
            className={`btn px-6 py-3 font-semibold ${buttonBg} disabled:opacity-50 disabled:cursor-not-allowed`}
          >
            {isLoading ? 'Subscribing...' : 'Subscribe'}
          </button>
        </form>
        <p className={`text-sm ${accentColor} mt-4`}>
          No spam, unsubscribe at any time.
        </p>
      </div>
    </section>
  )
}