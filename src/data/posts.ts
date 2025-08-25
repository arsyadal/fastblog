import { Post, PostMeta } from '@/types/post'

// Sample blog posts data
const posts: Post[] = [
  {
    slug: 'getting-started-with-nextjs',
    title: 'Getting Started with Next.js: A Complete Guide',
    excerpt: 'Learn how to build modern web applications with Next.js, the React framework that provides an excellent developer experience and production-ready features.',
    content: `# Getting Started with Next.js

Next.js is a powerful React framework that makes building full-stack web applications simple and efficient. In this guide, we'll explore the fundamentals and get you up and running.

## Why Next.js?

Next.js provides several key benefits:

- **Server-Side Rendering (SSR)**: Better SEO and performance
- **Static Site Generation (SSG)**: Pre-render pages at build time
- **API Routes**: Build backend APIs within your Next.js app
- **File-based Routing**: Intuitive routing based on file structure
- **Built-in CSS Support**: CSS Modules, Sass, and more
- **TypeScript Support**: First-class TypeScript support

## Getting Started

1. Create a new Next.js project:
\`\`\`bash
npx create-next-app@latest my-app
cd my-app
npm run dev
\`\`\`

2. Open your browser and navigate to \`http://localhost:3000\`

## Key Concepts

### Pages and Routing

Next.js uses a file-based routing system. Create files in the \`pages\` directory to automatically create routes.

### Data Fetching

Next.js provides multiple ways to fetch data:

- \`getStaticProps\` for static generation
- \`getServerSideProps\` for server-side rendering
- \`getStaticPaths\` for dynamic routes

## Conclusion

Next.js is an excellent choice for building modern web applications. Its developer experience, performance optimizations, and extensive ecosystem make it a top choice for React developers.

Start building your next project with Next.js today!`,
    author: 'arsyadal',
    publishedAt: '2024-01-15T10:00:00Z',
    category: 'Development',
    tags: ['Next.js', 'React', 'JavaScript', 'Web Development'],
    coverImage: 'https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=800&h=400&fit=crop',
    readTime: 8,
    featured: true,
    seo: {
      title: 'Getting Started with Next.js: Complete Guide for Beginners',
      description: 'Learn how to build modern web applications with Next.js. Complete guide covering setup, routing, data fetching, and deployment.',
      keywords: ['Next.js', 'React', 'JavaScript', 'Web Development', 'Tutorial'],
      image: 'https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=1200&h=630&fit=crop'
    }
  },
  {
    slug: 'mastering-typescript',
    title: 'Mastering TypeScript: From Beginner to Advanced',
    excerpt: 'Dive deep into TypeScript and learn advanced patterns, best practices, and real-world applications that will make you a TypeScript expert.',
    content: `# Mastering TypeScript

TypeScript has become the standard for building large-scale JavaScript applications. This comprehensive guide will take you from basics to advanced concepts.

## What is TypeScript?

TypeScript is a superset of JavaScript that adds static typing, making your code more reliable and maintainable.

## Basic Types

\`\`\`typescript
let name: string = 'John'
let age: number = 30
let isActive: boolean = true
let hobbies: string[] = ['reading', 'coding']
let tuple: [string, number] = ['hello', 10]
\`\`\`

## Advanced Types

### Union Types
\`\`\`typescript
type Status = 'loading' | 'success' | 'error'
\`\`\`

### Generic Types
\`\`\`typescript
function identity<T>(arg: T): T {
  return arg
}
\`\`\`

## Best Practices

1. **Use strict mode**: Enable strict type checking
2. **Prefer interfaces over types**: For object shapes
3. **Use utility types**: Leverage built-in TypeScript utilities
4. **Avoid any**: Use proper typing instead

## Conclusion

TypeScript is an essential tool for modern JavaScript development. Its type system helps catch errors early and provides better developer experience.`,
    author: 'arsyadal',
    publishedAt: '2024-01-10T14:30:00Z',
    category: 'Development',
    tags: ['TypeScript', 'JavaScript', 'Programming', 'Web Development'],
    coverImage: 'https://images.unsplash.com/photo-1516116216624-53e697fedbea?w=800&h=400&fit=crop',
    readTime: 12,
    featured: true,
    seo: {
      title: 'Mastering TypeScript: Complete Guide for Developers',
      description: 'Learn TypeScript from basics to advanced concepts. Comprehensive guide with examples and best practices.',
      keywords: ['TypeScript', 'JavaScript', 'Programming', 'Web Development', 'Tutorial'],
      image: 'https://images.unsplash.com/photo-1516116216624-53e697fedbea?w=1200&h=630&fit=crop'
    }
  },
  {
    slug: 'modern-css-techniques',
    title: 'Modern CSS Techniques for Beautiful UIs',
    excerpt: 'Explore cutting-edge CSS techniques including Grid, Flexbox, Custom Properties, and modern animations to create stunning user interfaces.',
    content: `# Modern CSS Techniques

CSS has evolved significantly in recent years. Let's explore the latest techniques that make building beautiful UIs easier than ever.

## CSS Grid Layout

CSS Grid is a powerful layout system that allows you to create complex two-dimensional layouts.

\`\`\`css
.container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}
\`\`\`

## Flexbox

Flexbox is perfect for one-dimensional layouts and responsive design.

\`\`\`css
.flex-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
}
\`\`\`

## CSS Custom Properties

Custom properties (CSS variables) make your styles more maintainable.

\`\`\`css
:root {
  --primary-color: #0ea5e9;
  --secondary-color: #64748b;
  --border-radius: 0.5rem;
}

.button {
  background-color: var(--primary-color);
  border-radius: var(--border-radius);
}
\`\`\`

## Modern Animations

CSS animations and transitions create smooth, engaging user experiences.

\`\`\`css
.card {
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1);
}
\`\`\`

## Conclusion

Modern CSS provides powerful tools for creating beautiful, responsive, and maintainable user interfaces. Embrace these techniques to build better web experiences.`,
    author: 'arsyadal',
    publishedAt: '2024-01-05T09:15:00Z',
    category: 'Design',
    tags: ['CSS', 'Web Design', 'UI/UX', 'Frontend'],
    coverImage: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=800&h=400&fit=crop',
    readTime: 10,
    featured: false,
    seo: {
      title: 'Modern CSS Techniques for Beautiful User Interfaces',
      description: 'Learn modern CSS techniques including Grid, Flexbox, Custom Properties, and animations for stunning UIs.',
      keywords: ['CSS', 'Web Design', 'UI/UX', 'Frontend', 'Web Development'],
      image: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=1200&h=630&fit=crop'
    }
  }
]

// Function to get all posts
export async function getAllPosts(): Promise<Post[]> {
  // In a real app, this would fetch from an API or database
  return posts.sort((a, b) => new Date(b.publishedAt).getTime() - new Date(a.publishedAt).getTime())
}

// Function to get featured posts
export async function getFeaturedPosts(): Promise<Post[]> {
  return posts.filter(post => post.featured).slice(0, 3)
}

// Function to get post by slug
export async function getPostBySlug(slug: string): Promise<Post | null> {
  return posts.find(post => post.slug === slug) || null
}

// Function to get posts by category
export async function getPostsByCategory(category: string): Promise<Post[]> {
  return posts.filter(post => post.category?.toLowerCase() === category.toLowerCase())
}

// Function to get posts by author
export async function getPostsByAuthor(author: string): Promise<Post[]> {
  return posts.filter(post => post.author.toLowerCase() === author.toLowerCase())
}

// Function to search posts
export async function searchPosts(query: string): Promise<Post[]> {
  const lowercaseQuery = query.toLowerCase()
  return posts.filter(post => 
    post.title.toLowerCase().includes(lowercaseQuery) ||
    post.excerpt.toLowerCase().includes(lowercaseQuery) ||
    post.content.toLowerCase().includes(lowercaseQuery) ||
    post.tags?.some(tag => tag.toLowerCase().includes(lowercaseQuery))
  )
}

// Function to get post metadata (for lists)
export async function getAllPostMeta(): Promise<PostMeta[]> {
  return posts.map(post => ({
    slug: post.slug,
    title: post.title,
    excerpt: post.excerpt,
    author: post.author,
    publishedAt: post.publishedAt,
    category: post.category,
    coverImage: post.coverImage,
    readTime: post.readTime,
    featured: post.featured
  }))
}