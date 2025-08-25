import { render, screen } from '@testing-library/react'
import HomePage from '../page'

// Mock Next.js Link component
jest.mock('next/link', () => {
  return ({ children, href }: { children: React.ReactNode; href: string }) => {
    return <a href={href}>{children}</a>
  }
})

describe('HomePage', () => {
  it('renders the main heading', () => {
    render(<HomePage />)
    expect(screen.getByText('Welcome to FastBlog')).toBeInTheDocument()
  })

  it('renders the hero section', () => {
    render(<HomePage />)
    expect(screen.getByText(/A modern, fast blog platform/)).toBeInTheDocument()
  })

  it('renders featured posts section', () => {
    render(<HomePage />)
    expect(screen.getByText('Featured Posts')).toBeInTheDocument()
  })

  it('renders the FastBlog logo in header', () => {
    render(<HomePage />)
    expect(screen.getByText('FastBlog')).toBeInTheDocument()
  })

  it('renders navigation links', () => {
    render(<HomePage />)
    expect(screen.getByText('Home')).toBeInTheDocument()
    expect(screen.getByText('Blog')).toBeInTheDocument()
    expect(screen.getByText('About')).toBeInTheDocument()
    expect(screen.getByText('Contact')).toBeInTheDocument()
  })

  it('renders call-to-action buttons', () => {
    render(<HomePage />)
    expect(screen.getByText('Read Blog')).toBeInTheDocument()
    expect(screen.getByText('Learn More')).toBeInTheDocument()
  })
})