import Link from 'next/link'
import { Calendar, Clock, User, Tag } from 'lucide-react'
import { BlogPost } from '@/data/posts'

interface BlogCardProps {
  post: BlogPost
  variant?: 'default' | 'featured'
}

export default function BlogCard({ post, variant = 'default' }: BlogCardProps) {
  const isFeatured = variant === 'featured'

  return (
    <article className={`card hover:shadow-lg transition-all duration-300 group ${
      isFeatured ? 'md:col-span-2 lg:col-span-1' : ''
    }`}>
      <div className="p-6">
        <div className="flex items-center justify-between mb-3">
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-primary-100 text-primary-800">
            {post.category}
          </span>
          <span className="text-sm text-gray-500">{post.readTime}</span>
        </div>
        
        <h3 className={`font-semibold text-gray-900 mb-3 group-hover:text-primary-600 transition-colors ${
          isFeatured ? 'text-2xl' : 'text-xl'
        } line-clamp-2`}>
          <Link href={`/blog/${post.slug}`}>
            {post.title}
          </Link>
        </h3>
        
        <p className={`text-gray-600 mb-4 line-clamp-3 ${
          isFeatured ? 'text-lg' : 'text-base'
        }`}>
          {post.excerpt}
        </p>
        
        <div className="flex flex-wrap gap-2 mb-4">
          {post.tags.slice(0, 3).map((tag) => (
            <span
              key={tag}
              className="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors"
            >
              #{tag}
            </span>
          ))}
        </div>
        
        <div className="flex items-center justify-between text-sm text-gray-500">
          <div className="flex items-center space-x-4">
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
      </div>
    </article>
  )
}