import Header from '@/components/Header'
import Footer from '@/components/Footer'
import Newsletter from '@/components/Newsletter'

export default function AboutPage() {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header currentPage="about" />

      {/* Page Header */}
      <section className="bg-white py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">About FastBlog</h1>
          <p className="text-xl text-gray-600 max-w-2xl mx-auto">
            A modern blog platform built with cutting-edge technologies to share knowledge and ideas
          </p>
        </div>
      </section>

      {/* Main Content */}
      <section className="py-16">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="prose prose-lg max-w-none">
            <h2>Our Mission</h2>
            <p>
              FastBlog was created with a simple mission: to provide a fast, modern, and user-friendly platform 
              for sharing knowledge, insights, and ideas. We believe that great content deserves great technology.
            </p>

            <h2>What We Do</h2>
            <p>
              We focus on technology, development, design, and innovation. Our blog covers a wide range of topics 
              including web development, software engineering, UI/UX design, and emerging technologies.
            </p>

            <h2>Our Values</h2>
            <ul>
              <li><strong>Quality:</strong> We strive to publish high-quality, well-researched content</li>
              <li><strong>Innovation:</strong> We embrace new technologies and approaches</li>
              <li><strong>Community:</strong> We believe in building and nurturing a community of learners</li>
              <li><strong>Accessibility:</strong> We make our content accessible to everyone</li>
            </ul>

            <h2>About the Author</h2>
            <p>
              Hi, I'm <strong>arsyadal</strong>, a passionate developer and technology enthusiast. I love exploring 
              new technologies, sharing knowledge, and helping others learn and grow in their development journey.
            </p>

            <p>
              With years of experience in web development, I've worked with various technologies including React, 
              Next.js, TypeScript, and many more. This blog is my way of giving back to the community and sharing 
              what I've learned along the way.
            </p>

            <h2>Technology Stack</h2>
            <p>
              FastBlog is built with modern web technologies to ensure the best performance and user experience:
            </p>
            <ul>
              <li><strong>Frontend:</strong> Next.js 14, React 18, TypeScript</li>
              <li><strong>Styling:</strong> Tailwind CSS with custom design system</li>
              <li><strong>Icons:</strong> Lucide React for beautiful, consistent icons</li>
              <li><strong>Performance:</strong> Optimized for speed and SEO</li>
            </ul>

            <h2>Get in Touch</h2>
            <p>
              Have a question, suggestion, or just want to say hello? I'd love to hear from you! You can reach out 
              through our contact page or connect with me on social media.
            </p>

            <h2>Support the Blog</h2>
            <p>
              If you find our content valuable, consider supporting us by:
            </p>
            <ul>
              <li>Sharing our articles with your network</li>
              <li>Leaving comments and engaging with our content</li>
              <li>Following us on social media</li>
              <li>Subscribing to our newsletter</li>
            </ul>
          </div>
        </div>
      </section>

      {/* Newsletter */}
      <Newsletter 
        variant="secondary"
        title="Join Our Community"
        description="Stay connected and never miss our latest articles and insights"
      />

      <Footer />
    </div>
  )
}