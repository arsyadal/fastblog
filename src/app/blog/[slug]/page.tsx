import Link from 'next/link'
import { notFound } from 'next/navigation'
import { Calendar, Clock, User, Tag, ArrowLeft, Share2, Bookmark, Heart } from 'lucide-react'

// This would typically come from a CMS or database
const getBlogPost = (slug: string) => {
  const posts = {
    'getting-started-nextjs-14': {
      title: 'Getting Started with Next.js 14',
      excerpt: 'Learn how to build modern web applications with the latest version of Next.js and explore its new features.',
      content: `
        <h2>Introduction</h2>
        <p>Next.js 14 brings exciting new features and improvements that make building modern web applications faster and more efficient than ever before. In this comprehensive guide, we'll explore everything you need to know to get started with Next.js 14.</p>
        
        <h2>What's New in Next.js 14</h2>
        <p>The latest version introduces several groundbreaking features:</p>
        <ul>
          <li><strong>App Router:</strong> A new file-system based router that makes routing more intuitive</li>
          <li><strong>Server Components:</strong> React components that run on the server for better performance</li>
          <li><strong>Turbopack:</strong> An incremental bundler that's up to 700x faster than Webpack</li>
          <li><strong>Partial Prerendering:</strong> Combines static and dynamic rendering for optimal performance</li>
        </ul>
        
        <h2>Setting Up Your First Project</h2>
        <p>Getting started with Next.js 14 is incredibly simple. You can create a new project using the following command:</p>
        <pre><code>npx create-next-app@latest my-app --typescript --tailwind --app</code></pre>
        
        <p>This command will create a new Next.js project with:</p>
        <ul>
          <li>TypeScript support for better type safety</li>
          <li>Tailwind CSS for styling</li>
          <li>App Router enabled by default</li>
          <li>ESLint configuration</li>
        </ul>
        
        <h2>Understanding the App Router</h2>
        <p>The App Router is one of the most significant changes in Next.js 14. It's based on React Server Components and provides a more intuitive way to organize your application:</p>
        
        <pre><code>app/
├── layout.tsx          # Root layout
├── page.tsx            # Home page
├── blog/
│   ├── page.tsx        # Blog listing
│   └── [slug]/
│       └── page.tsx    # Individual blog post
└── globals.css         # Global styles</code></pre>
        
        <h2>Server Components vs Client Components</h2>
        <p>Next.js 14 introduces a new paradigm with Server Components. By default, all components in the App Router are Server Components, which means they run on the server and can:</p>
        <ul>
          <li>Access backend resources directly</li>
          <li>Keep sensitive data on the server</li>
          <li>Reduce the JavaScript bundle size</li>
          <li>Improve initial page load performance</li>
        </ul>
        
        <p>To use Client Components (components that run in the browser), you need to add the <code>'use client'</code> directive at the top of your file:</p>
        
        <pre><code>'use client'

import { useState } from 'react'

export default function Counter() {
  const [count, setCount] = useState(0)
  
  return (
    <button onClick={() => setCount(count + 1)}>
      Count: {count}
    </button>
  )
}</code></pre>
        
        <h2>Data Fetching</h2>
        <p>Next.js 14 makes data fetching incredibly simple with async Server Components. You can fetch data directly in your components:</p>
        
        <pre><code>async function BlogPost({ params }: { params: { slug: string } }) {
  const post = await fetchPost(params.slug)
  
  return (
    <article>
      <h1>{post.title}</h1>
      <p>{post.content}</p>
    </article>
  )
}</code></pre>
        
        <h2>Performance Optimizations</h2>
        <p>Next.js 14 includes several performance optimizations out of the box:</p>
        <ul>
          <li><strong>Automatic Image Optimization:</strong> Images are automatically optimized and served in modern formats</li>
          <li><strong>Font Optimization:</strong> Google Fonts are automatically optimized and self-hosted</li>
          <li><strong>Bundle Analysis:</strong> Built-in bundle analyzer to identify optimization opportunities</li>
          <li><strong>Incremental Static Regeneration:</strong> Pages can be updated after build time</li>
        </ul>
        
        <h2>Deployment</h2>
        <p>Deploying your Next.js 14 application is straightforward. You can deploy to Vercel (the creators of Next.js) with zero configuration, or to any other platform that supports Node.js.</p>
        
        <h2>Conclusion</h2>
        <p>Next.js 14 represents a significant leap forward in the React ecosystem. With its new App Router, Server Components, and performance optimizations, it's the perfect framework for building modern, scalable web applications.</p>
        
        <p>Whether you're building a personal blog, an e-commerce site, or a complex web application, Next.js 14 provides the tools and performance you need to succeed.</p>
      `,
      author: 'arsyadal',
      date: '2024-01-15',
      readTime: '5 min read',
      category: 'Development',
      tags: ['Next.js', 'React', 'Web Development', 'TypeScript'],
      slug: 'getting-started-nextjs-14'
    }
  }
  
  return posts[slug as keyof typeof posts]
}

export default function BlogPostPage({ params }: { params: { slug: string } }) {
  const post = getBlogPost(params.slug)
  
  if (!post) {
    notFound()
  }

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

      {/* Article Header */}
      <section className="bg-white py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="mb-8">
            <Link 
              href="/blog" 
              className="inline-flex items-center text-primary-600 hover:text-primary-700 transition-colors mb-6"
            >
              <ArrowLeft className="h-4 w-4 mr-2" />
              Back to Blog
            </Link>
            
            <div className="flex items-center space-x-4 mb-4">
              <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-primary-100 text-primary-800">
                {post.category}
              </span>
              <span className="text-sm text-gray-500">{post.readTime}</span>
            </div>
            
            <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-6 leading-tight">
              {post.title}
            </h1>
            
            <p className="text-xl text-gray-600 mb-8 leading-relaxed">
              {post.excerpt}
            </p>
            
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-6 text-sm text-gray-500">
                <div className="flex items-center">
                  <User className="h-4 w-4 mr-2" />
                  {post.author}
                </div>
                <div className="flex items-center">
                  <Calendar className="h-4 w-4 mr-2" />
                  {new Date(post.date).toLocaleDateString('en-US', {
                    year: 'numeric',
                    month: 'long',
                    day: 'numeric'
                  })}
                </div>
              </div>
              
              <div className="flex items-center space-x-4">
                <button className="p-2 text-gray-400 hover:text-gray-600 transition-colors">
                  <Heart className="h-5 w-5" />
                </button>
                <button className="p-2 text-gray-400 hover:text-gray-600 transition-colors">
                  <Bookmark className="h-5 w-5" />
                </button>
                <button className="p-2 text-gray-400 hover:text-gray-600 transition-colors">
                  <Share2 className="h-5 w-5" />
                </button>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Article Content */}
      <section className="py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <article className="prose prose-lg max-w-none">
            <div dangerouslySetInnerHTML={{ __html: post.content }} />
          </article>
          
          {/* Tags */}
          <div className="mt-12 pt-8 border-t border-gray-200">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Tags</h3>
            <div className="flex flex-wrap gap-2">
              {post.tags.map((tag) => (
                <span
                  key={tag}
                  className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors"
                >
                  #{tag}
                </span>
              ))}
            </div>
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