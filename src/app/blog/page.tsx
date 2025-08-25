import Link from 'next/link'
import { Search, Filter, Calendar, Clock, User, Tag } from 'lucide-react'

const blogPosts = [
  {
    id: 1,
    title: 'Getting Started with Next.js 14',
    excerpt: 'Learn how to build modern web applications with the latest version of Next.js and explore its new features.',
    author: 'arsyadal',
    date: '2024-01-15',
    readTime: '5 min read',
    category: 'Development',
    tags: ['Next.js', 'React', 'Web Development'],
    slug: 'getting-started-nextjs-14'
  },
  {
    id: 2,
    title: 'The Future of Web Development',
    excerpt: 'Explore the latest trends and technologies that are shaping the future of web development.',
    author: 'arsyadal',
    date: '2024-01-10',
    readTime: '8 min read',
    category: 'Technology',
    tags: ['Web Development', 'Trends', 'Innovation'],
    slug: 'future-web-development'
  },
  {
    id: 3,
    title: 'Building Scalable React Applications',
    excerpt: 'Discover best practices and patterns for building large-scale React applications that are maintainable and performant.',
    author: 'arsyadal',
    date: '2024-01-05',
    readTime: '12 min read',
    category: 'Development',
    tags: ['React', 'Architecture', 'Best Practices'],
    slug: 'building-scalable-react-apps'
  },
  {
    id: 4,
    title: 'Mastering TypeScript for React',
    excerpt: 'A comprehensive guide to using TypeScript effectively in React applications for better type safety and developer experience.',
    author: 'arsyadal',
    date: '2024-01-01',
    readTime: '15 min read',
    category: 'Development',
    tags: ['TypeScript', 'React', 'Type Safety'],
    slug: 'mastering-typescript-react'
  },
  {
    id: 5,
    title: 'CSS Grid vs Flexbox: When to Use What',
    excerpt: 'Understanding the differences between CSS Grid and Flexbox and choosing the right layout method for your design.',
    author: 'arsyadal',
    date: '2023-12-28',
    readTime: '10 min read',
    category: 'Design',
    tags: ['CSS', 'Grid', 'Flexbox', 'Layout'],
    slug: 'css-grid-vs-flexbox'
  },
  {
    id: 6,
    title: 'Performance Optimization Techniques',
    excerpt: 'Learn advanced techniques to optimize your web applications for better performance and user experience.',
    author: 'arsyadal',
    date: '2023-12-20',
    readTime: '18 min read',
    category: 'Development',
    tags: ['Performance', 'Optimization', 'Web Apps'],
    slug: 'performance-optimization-techniques'
  }
]

const categories = ['All', 'Development', 'Technology', 'Design', 'Tutorials']
const tags = ['Next.js', 'React', 'TypeScript', 'CSS', 'Performance', 'Web Development']

export default function BlogPage() {
  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center">
              <Link href="/" className="text-2xl font-bold text-gray-900 hover:text-primary-600 transition-colors">
                FastBlog
              </Link>
            </div>
            <nav className="hidden md:flex space-x-8">
              <Link href="/" className="text-gray-700 hover:text-primary-600 px-3 py-2 text-sm font-medium">
                Home
              </Link>
              <Link href="/blog" className="text-primary-600 px-3 py-2 text-sm font-medium">
                Blog
              </Link>
              <Link href="/about" className="text-gray-700 hover:text-primary-600 px-3 py-2 text-sm font-medium">
                About
              </Link>
              <Link href="/contact" className="text-gray-700 hover:text-primary-600 px-3 py-2 text-sm font-medium">
                Contact
              </Link>
            </nav>
          </div>
        </div>
      </header>

      {/* Page Header */}
      <section className="bg-white py-16">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">Blog</h1>
          <p className="text-xl text-gray-600 max-w-2xl mx-auto">
            Discover insights, tutorials, and thoughts on technology, development, and innovation
          </p>
        </div>
      </section>

      {/* Search and Filters */}
      <section className="bg-white border-b border-gray-200 py-8">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex flex-col md:flex-row gap-6 items-center justify-between">
            {/* Search */}
            <div className="relative flex-1 max-w-md">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
              <input
                type="text"
                placeholder="Search articles..."
                className="input pl-10 w-full"
              />
            </div>

            {/* Category Filter */}
            <div className="flex items-center space-x-4">
              <span className="text-sm font-medium text-gray-700">Categories:</span>
              <div className="flex space-x-2">
                {categories.map((category) => (
                  <button
                    key={category}
                    className={`px-3 py-1 rounded-full text-sm font-medium transition-colors ${
                      category === 'All'
                        ? 'bg-primary-100 text-primary-800'
                        : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                    }`}
                  >
                    {category}
                  </button>
                ))}
              </div>
            </div>
          </div>

          {/* Tags */}
          <div className="mt-6 flex items-center space-x-4">
            <span className="text-sm font-medium text-gray-700">Popular tags:</span>
            <div className="flex flex-wrap gap-2">
              {tags.map((tag) => (
                <button
                  key={tag}
                  className="px-2 py-1 bg-gray-100 text-gray-700 text-xs rounded hover:bg-gray-200 transition-colors"
                >
                  #{tag}
                </button>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* Blog Posts */}
      <section className="py-16">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
            {blogPosts.map((post) => (
              <article key={post.id} className="card hover:shadow-lg transition-all duration-300 group">
                <div className="p-6">
                  <div className="flex items-center justify-between mb-3">
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-primary-100 text-primary-800">
                      {post.category}
                    </span>
                    <span className="text-sm text-gray-500">{post.readTime}</span>
                  </div>
                  
                  <h3 className="text-xl font-semibold text-gray-900 mb-3 group-hover:text-primary-600 transition-colors">
                    <Link href={`/blog/${post.slug}`}>
                      {post.title}
                    </Link>
                  </h3>
                  
                  <p className="text-gray-600 mb-4 line-clamp-3">
                    {post.excerpt}
                  </p>
                  
                  <div className="flex flex-wrap gap-2 mb-4">
                    {post.tags.slice(0, 3).map((tag) => (
                      <span
                        key={tag}
                        className="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-gray-100 text-gray-700"
                      >
                        #{tag}
                      </span>
                    ))}
                  </div>
                  
                  <div className="flex items-center justify-between text-sm text-gray-500">
                    <div className="flex items-center">
                      <User className="h-4 w-4 mr-1" />
                      {post.author}
                    </div>
                    <div className="flex items-center">
                      <Calendar className="h-4 w-4 mr-1" />
                      {new Date(post.date).toLocaleDateString()}
                    </div>
                  </div>
                </div>
              </article>
            ))}
          </div>

          {/* Pagination */}
          <div className="mt-16 flex justify-center">
            <nav className="flex items-center space-x-2">
              <button className="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50">
                Previous
              </button>
              <button className="px-3 py-2 text-sm font-medium text-white bg-primary-600 border border-primary-600 rounded-md">
                1
              </button>
              <button className="px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50">
                2
              </button>
              <button className="px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50">
                3
              </button>
              <button className="px-3 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-md hover:bg-gray-50">
                Next
              </button>
            </nav>
          </div>
        </div>
      </section>

      {/* Newsletter Signup */}
      <section className="bg-primary-600 py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl font-bold text-white mb-4">
            Stay Updated
          </h2>
          <p className="text-xl text-primary-100 mb-8">
            Get the latest articles and insights delivered to your inbox
          </p>
          <div className="flex flex-col sm:flex-row gap-4 max-w-md mx-auto">
            <input
              type="email"
              placeholder="Enter your email"
              className="input flex-1"
            />
            <button className="btn bg-white text-primary-700 hover:bg-gray-100 px-6 py-3 font-semibold">
              Subscribe
            </button>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-900 text-white py-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-4 gap-8">
            <div>
              <h3 className="text-xl font-bold mb-4">FastBlog</h3>
              <p className="text-gray-400">
                A modern blog platform for sharing knowledge and ideas with the world.
              </p>
            </div>
            
            <div>
              <h4 className="text-lg font-semibold mb-4">Quick Links</h4>
              <ul className="space-y-2">
                <li><Link href="/" className="text-gray-400 hover:text-white transition-colors">Home</Link></li>
                <li><Link href="/blog" className="text-gray-400 hover:text-white transition-colors">Blog</Link></li>
                <li><Link href="/about" className="text-gray-400 hover:text-white transition-colors">About</Link></li>
                <li><Link href="/contact" className="text-gray-400 hover:text-white transition-colors">Contact</Link></li>
              </ul>
            </div>
            
            <div>
              <h4 className="text-lg font-semibold mb-4">Categories</h4>
              <ul className="space-y-2">
                <li><Link href="/blog?category=development" className="text-gray-400 hover:text-white transition-colors">Development</Link></li>
                <li><Link href="/blog?category=technology" className="text-gray-400 hover:text-white transition-colors">Technology</Link></li>
                <li><Link href="/blog?category=design" className="text-gray-400 hover:text-white transition-colors">Design</Link></li>
                <li><Link href="/blog?category=tutorials" className="text-gray-400 hover:text-white transition-colors">Tutorials</Link></li>
              </ul>
            </div>
            
            <div>
              <h4 className="text-lg font-semibold mb-4">Connect</h4>
              <ul className="space-y-2">
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Twitter</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">GitHub</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">LinkedIn</a></li>
                <li><a href="#" className="text-gray-400 hover:text-white transition-colors">Email</a></li>
              </ul>
            </div>
          </div>
          
          <div className="border-t border-gray-800 mt-8 pt-8 text-center">
            <p className="text-gray-400">
              © 2024 FastBlog. All rights reserved. Built with ❤️ by arsyadal.
            </p>
          </div>
        </div>
      </footer>
    </div>
  )
}