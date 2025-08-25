export interface Post {
  slug: string
  title: string
  excerpt: string
  content: string
  author: string
  publishedAt: string
  updatedAt?: string
  category?: string
  tags?: string[]
  coverImage?: string
  readTime: number
  featured?: boolean
  seo?: {
    title?: string
    description?: string
    keywords?: string[]
    image?: string
  }
}

export interface PostMeta {
  slug: string
  title: string
  excerpt: string
  author: string
  publishedAt: string
  category?: string
  coverImage?: string
  readTime: number
  featured?: boolean
}

export interface Category {
  name: string
  slug: string
  description?: string
  postCount: number
}

export interface Author {
  name: string
  bio?: string
  avatar?: string
  social?: {
    twitter?: string
    github?: string
    linkedin?: string
    website?: string
  }
}

export interface BlogFilters {
  category?: string
  author?: string
  tags?: string[]
  search?: string
  featured?: boolean
  sortBy?: 'date' | 'title' | 'readTime'
  sortOrder?: 'asc' | 'desc'
}