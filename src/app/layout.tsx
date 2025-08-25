import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import '../styles/globals.css'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'FastBlog - A Modern Blog Platform',
  description: 'A fast, modern blog application built with Next.js and React',
  keywords: ['blog', 'nextjs', 'react', 'typescript', 'fast'],
  authors: [{ name: 'arsyadal' }],
  creator: 'arsyadal',
  openGraph: {
    type: 'website',
    locale: 'en_US',
    url: 'https://fastblog.com',
    title: 'FastBlog - A Modern Blog Platform',
    description: 'A fast, modern blog application built with Next.js and React',
    siteName: 'FastBlog',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'FastBlog - A Modern Blog Platform',
    description: 'A fast, modern blog application built with Next.js and React',
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="h-full">
      <body className={`${inter.className} h-full`}>
        {children}
      </body>
    </html>
  )
}