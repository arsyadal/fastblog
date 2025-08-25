export interface BlogPost {
  id: number
  title: string
  excerpt: string
  content: string
  author: string
  date: string
  readTime: string
  category: string
  tags: string[]
  slug: string
  featured?: boolean
}

export const blogPosts: BlogPost[] = [
  {
    id: 1,
    title: 'Getting Started with Next.js 14',
    excerpt: 'Learn how to build modern web applications with the latest version of Next.js and explore its new features.',
    content: `
      <h2>Introduction</h2>
      <p>Next.js 14 brings exciting new features and improvements that make building modern web applications faster and more efficient than ever before.</p>
      
      <h2>What's New in Next.js 14</h2>
      <p>The latest version introduces several groundbreaking features:</p>
      <ul>
        <li><strong>App Router:</strong> A new file-system based router that makes routing more intuitive</li>
        <li><strong>Server Components:</strong> React components that run on the server for better performance</li>
        <li><strong>Turbopack:</strong> An incremental bundler that's up to 700x faster than Webpack</li>
      </ul>
    `,
    author: 'arsyadal',
    date: '2024-01-15',
    readTime: '5 min read',
    category: 'Development',
    tags: ['Next.js', 'React', 'Web Development'],
    slug: 'getting-started-nextjs-14',
    featured: true
  },
  {
    id: 2,
    title: 'The Future of Web Development',
    excerpt: 'Explore the latest trends and technologies that are shaping the future of web development.',
    content: `
      <h2>Introduction</h2>
      <p>Web development is evolving at an unprecedented pace, with new technologies and frameworks emerging constantly.</p>
      
      <h2>Key Trends</h2>
      <p>Several key trends are shaping the future of web development:</p>
      <ul>
        <li><strong>AI-Powered Development:</strong> Tools that assist developers in writing better code</li>
        <li><strong>WebAssembly:</strong> Bringing near-native performance to the web</li>
        <li><strong>Progressive Web Apps:</strong> Blurring the line between web and native apps</li>
      </ul>
    `,
    author: 'arsyadal',
    date: '2024-01-10',
    readTime: '8 min read',
    category: 'Technology',
    tags: ['Web Development', 'Trends', 'Innovation'],
    slug: 'future-web-development',
    featured: true
  },
  {
    id: 3,
    title: 'Building Scalable React Applications',
    excerpt: 'Discover best practices and patterns for building large-scale React applications that are maintainable and performant.',
    content: `
      <h2>Introduction</h2>
      <p>Building scalable React applications requires careful consideration of architecture, state management, and performance.</p>
      
      <h2>Architecture Patterns</h2>
      <p>Several architectural patterns can help with scalability:</p>
      <ul>
        <li><strong>Component Composition:</strong> Building complex UIs from simple, reusable components</li>
        <li><strong>State Management:</strong> Choosing the right state management solution for your app</li>
        <li><strong>Code Splitting:</strong> Breaking your app into smaller, loadable chunks</li>
      </ul>
    `,
    author: 'arsyadal',
    date: '2024-01-05',
    readTime: '12 min read',
    category: 'Development',
    tags: ['React', 'Architecture', 'Best Practices'],
    slug: 'building-scalable-react-apps',
    featured: true
  },
  {
    id: 4,
    title: 'Mastering TypeScript for React',
    excerpt: 'A comprehensive guide to using TypeScript effectively in React applications for better type safety and developer experience.',
    content: `
      <h2>Introduction</h2>
      <p>TypeScript adds static typing to JavaScript, making React applications more robust and maintainable.</p>
      
      <h2>Benefits of TypeScript</h2>
      <p>TypeScript provides several key benefits:</p>
      <ul>
        <li><strong>Type Safety:</strong> Catch errors at compile time rather than runtime</li>
        <li><strong>Better IntelliSense:</strong> Enhanced autocomplete and documentation</li>
        <li><strong>Refactoring Support:</strong> Safe refactoring with confidence</li>
      </ul>
    `,
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
    content: `
      <h2>Introduction</h2>
      <p>CSS Grid and Flexbox are powerful layout systems that solve different layout problems.</p>
      
      <h2>When to Use Grid</h2>
      <p>CSS Grid is ideal for:</p>
      <ul>
        <li><strong>Two-dimensional layouts:</strong> Both rows and columns</li>
        <li><strong>Complex page layouts:</strong> Overall page structure</li>
        <li><strong>Grid-based designs:</strong> Card layouts, photo galleries</li>
      </ul>
    `,
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
    content: `
      <h2>Introduction</h2>
      <p>Performance optimization is crucial for providing a great user experience and improving SEO rankings.</p>
      
      <h2>Key Optimization Areas</h2>
      <p>Focus on these key areas for performance improvement:</p>
      <ul>
        <li><strong>Bundle Size:</strong> Reduce JavaScript and CSS bundle sizes</li>
        <li><strong>Image Optimization:</strong> Use appropriate formats and sizes</li>
        <li><strong>Caching:</strong> Implement effective caching strategies</li>
      </ul>
    `,
    author: 'arsyadal',
    date: '2023-12-20',
    readTime: '18 min read',
    category: 'Development',
    tags: ['Performance', 'Optimization', 'Web Apps'],
    slug: 'performance-optimization-techniques'
  }
]

export function getFeaturedPosts(): BlogPost[] {
  return blogPosts.filter(post => post.featured)
}

export function getPostsByCategory(category: string): BlogPost[] {
  if (category === 'All') return blogPosts
  return blogPosts.filter(post => post.category === category)
}

export function getPostsByTag(tag: string): BlogPost[] {
  return blogPosts.filter(post => post.tags.includes(tag))
}

export function getPostBySlug(slug: string): BlogPost | undefined {
  return blogPosts.find(post => post.slug === slug)
}

export function searchPosts(query: string): BlogPost[] {
  const lowercaseQuery = query.toLowerCase()
  return blogPosts.filter(post => 
    post.title.toLowerCase().includes(lowercaseQuery) ||
    post.excerpt.toLowerCase().includes(lowercaseQuery) ||
    post.tags.some(tag => tag.toLowerCase().includes(lowercaseQuery))
  )
}