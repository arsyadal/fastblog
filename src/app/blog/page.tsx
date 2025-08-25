import { Suspense } from 'react'
import { getAllPostMeta } from '@/data/posts'
import BlogList from '@/components/BlogList'
import BlogFilters from '@/components/BlogFilters'
import Header from '@/components/Header'
import Footer from '@/components/Footer'

export const metadata = {
  title: 'Blog - FastBlog',
  description: 'Explore our collection of articles, tutorials, and insights on modern web development, design, and technology.',
}

export default async function BlogPage() {
  const posts = await getAllPostMeta()

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <Header />
      
      <main className="container mx-auto px-4 py-8">
        {/* Page Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-4">
            Our Blog
          </h1>
          <p className="text-xl text-gray-600 dark:text-gray-300 max-w-3xl mx-auto">
            Discover insights, tutorials, and stories from our team of developers and designers
          </p>
        </div>

        {/* Filters and Search */}
        <BlogFilters />

        {/* Blog Posts */}
        <Suspense fallback={<div>Loading posts...</div>}>
          <BlogList posts={posts} />
        </Suspense>
      </main>
      
      <Footer />
    </div>
  )
}