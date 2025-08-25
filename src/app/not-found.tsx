import Link from 'next/link'
import { Home, ArrowLeft } from 'lucide-react'
import Header from '@/components/Header'
import Footer from '@/components/Footer'

export default function NotFound() {
  return (
    <div className="min-h-screen bg-white dark:bg-gray-900">
      <Header />
      
      <main className="container mx-auto px-4 py-8">
        <div className="text-center py-20">
          {/* 404 Icon */}
          <div className="mb-8">
            <div className="w-32 h-32 bg-gradient-to-r from-primary-600 to-primary-400 rounded-full flex items-center justify-center mx-auto">
              <span className="text-white text-6xl font-bold">404</span>
            </div>
          </div>
          
          {/* Error Message */}
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-6">
            Page Not Found
          </h1>
          <p className="text-xl text-gray-600 dark:text-gray-300 mb-8 max-w-2xl mx-auto">
            Oops! The page you're looking for doesn't exist. It might have been moved, deleted, or you entered the wrong URL.
          </p>
          
          {/* Action Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link 
              href="/" 
              className="btn-primary text-lg px-8 py-4 inline-flex items-center gap-2"
            >
              <Home className="w-5 h-5" />
              Go Home
            </Link>
            <button 
              onClick={() => window.history.back()} 
              className="btn-secondary text-lg px-8 py-4 inline-flex items-center gap-2"
            >
              <ArrowLeft className="w-5 h-5" />
              Go Back
            </button>
          </div>
          
          {/* Helpful Links */}
          <div className="mt-12">
            <p className="text-gray-500 dark:text-gray-400 mb-4">Or try these helpful links:</p>
            <div className="flex flex-wrap justify-center gap-4">
              <Link 
                href="/blog" 
                className="text-primary-600 dark:text-primary-400 hover:underline"
              >
                Browse Blog
              </Link>
              <Link 
                href="/about" 
                className="text-primary-600 dark:text-primary-400 hover:underline"
              >
                About Us
              </Link>
              <Link 
                href="/contact" 
                className="text-primary-600 dark:text-primary-400 hover:underline"
              >
                Contact
              </Link>
            </div>
          </div>
        </div>
      </main>
      
      <Footer />
    </div>
  )
}