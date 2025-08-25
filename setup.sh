#!/bin/bash

echo "🚀 Setting up FastBlog..."

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Install additional Tailwind CSS plugin
echo "🎨 Installing Tailwind CSS typography plugin..."
npm install @tailwindcss/typography

echo "✅ Setup complete!"
echo ""
echo "To start the development server, run:"
echo "  npm run dev"
echo ""
echo "Then open your browser and navigate to:"
echo "  http://localhost:3000"
echo ""
echo "Happy blogging! 🎉"