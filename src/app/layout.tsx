import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import '@/styles/globals.css'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'FastBlog - Modern Blog Platform',
  description: 'A modern, fast blog application built with Next.js and React',
  keywords: ['blog', 'nextjs', 'react', 'typescript', 'modern'],
  authors: [{ name: 'arsyadal' }],
  creator: 'arsyadal',
  openGraph: {
    type: 'website',
    locale: 'en_US',
    url: 'https://fastblog.com',
    title: 'FastBlog - Modern Blog Platform',
    description: 'A modern, fast blog application built with Next.js and React',
    siteName: 'FastBlog',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'FastBlog - Modern Blog Platform',
    description: 'A modern, fast blog application built with Next.js and React',
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="scroll-smooth">
      <body className={`${inter.className} antialiased`}>
        {children}
      </body>
    </html>
  )
}