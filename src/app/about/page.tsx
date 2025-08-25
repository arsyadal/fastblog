import Header from '@/components/Header'
import Footer from '@/components/Footer'

export const metadata = {
  title: 'About - FastBlog',
  description: 'Learn more about FastBlog, our mission, and the team behind the platform.',
}

export default function AboutPage() {
  return (
    <div className="min-h-screen bg-white dark:bg-gray-900">
      <Header />
      
      <main className="container mx-auto px-4 py-8">
        {/* Hero Section */}
        <section className="text-center py-20">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 dark:text-white mb-6">
            About FastBlog
          </h1>
          <p className="text-xl text-gray-600 dark:text-gray-300 max-w-3xl mx-auto">
            We're building the future of content creation and sharing, one blog post at a time.
          </p>
        </section>

        {/* Mission Section */}
        <section className="py-16">
          <div className="max-w-4xl mx-auto">
            <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center">
              Our Mission
            </h2>
            <div className="prose prose-lg prose-gray dark:prose-invert max-w-none">
              <p className="text-lg text-gray-600 dark:text-gray-300 leading-relaxed">
                FastBlog was born from a simple idea: blogging should be fast, beautiful, and accessible to everyone. 
                In today's digital world, content creators need tools that keep up with their creativity and don't 
                slow them down.
              </p>
              <p className="text-lg text-gray-600 dark:text-gray-300 leading-relaxed mt-6">
                We believe that great content deserves a great platform. That's why we've built FastBlog with 
                modern technologies like Next.js, React, and TypeScript, ensuring lightning-fast performance 
                and an exceptional user experience.
              </p>
            </div>
          </div>
        </section>

        {/* Values Section */}
        <section className="py-16 bg-gray-50 dark:bg-gray-800">
          <div className="max-w-6xl mx-auto">
            <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-12 text-center">
              Our Values
            </h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
              <div className="text-center">
                <div className="w-16 h-16 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-2xl">⚡</span>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                  Performance First
                </h3>
                <p className="text-gray-600 dark:text-gray-300">
                  We prioritize speed and performance in everything we build, ensuring your content loads quickly for readers worldwide.
                </p>
              </div>
              
              <div className="text-center">
                <div className="w-16 h-16 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-2xl">🎨</span>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                  Beautiful Design
                </h3>
                <p className="text-gray-600 dark:text-gray-300">
                  We believe that great content deserves beautiful presentation, with clean, modern designs that enhance readability.
                </p>
              </div>
              
              <div className="text-center">
                <div className="w-16 h-16 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center mx-auto mb-4">
                  <span className="text-2xl">🔧</span>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                  Developer Experience
                </h3>
                <p className="text-gray-600 dark:text-gray-300">
                  Built by developers, for developers. We focus on creating an intuitive and powerful platform for content creators.
                </p>
              </div>
            </div>
          </div>
        </section>

        {/* Technology Section */}
        <section className="py-16">
          <div className="max-w-4xl mx-auto">
            <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center">
              Built with Modern Technology
            </h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
              <div className="text-center p-4">
                <div className="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center mx-auto mb-3">
                  <span className="text-2xl font-bold text-gray-600 dark:text-gray-400">N</span>
                </div>
                <h4 className="font-semibold text-gray-900 dark:text-white">Next.js</h4>
              </div>
              
              <div className="text-center p-4">
                <div className="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center mx-auto mb-3">
                  <span className="text-2xl font-bold text-gray-600 dark:text-gray-400">R</span>
                </div>
                <h4 className="font-semibold text-gray-900 dark:text-white">React</h4>
              </div>
              
              <div className="text-center p-4">
                <div className="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center mx-auto mb-3">
                  <span className="text-2xl font-bold text-gray-600 dark:text-gray-400">T</span>
                </div>
                <h4 className="font-semibold text-gray-900 dark:text-white">TypeScript</h4>
              </div>
              
              <div className="text-center p-4">
                <div className="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center mx-auto mb-3">
                  <span className="text-2xl font-bold text-gray-600 dark:text-gray-400">T</span>
                </div>
                <h4 className="font-semibold text-gray-900 dark:text-white">Tailwind CSS</h4>
              </div>
            </div>
          </div>
        </section>

        {/* Team Section */}
        <section className="py-16 bg-gray-50 dark:bg-gray-800">
          <div className="max-w-4xl mx-auto">
            <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-8 text-center">
              Meet the Team
            </h2>
            <div className="text-center">
              <div className="w-24 h-24 bg-gradient-to-r from-primary-600 to-primary-400 rounded-full flex items-center justify-center mx-auto mb-4">
                <span className="text-white text-2xl font-bold">A</span>
              </div>
              <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-2">
                arsyadal
              </h3>
              <p className="text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
                Full-stack developer and creator of FastBlog. Passionate about building fast, 
                beautiful, and user-friendly web applications that make a difference.
              </p>
            </div>
          </div>
        </section>

        {/* Contact Section */}
        <section className="py-16">
          <div className="max-w-2xl mx-auto text-center">
            <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-4">
              Get in Touch
            </h2>
            <p className="text-lg text-gray-600 dark:text-gray-300 mb-8">
              Have questions, suggestions, or want to contribute? We'd love to hear from you!
            </p>
            <a
              href="/contact"
              className="btn-primary text-lg px-8 py-4"
            >
              Contact Us
            </a>
          </div>
        </section>
      </main>
      
      <Footer />
    </div>
  )
}