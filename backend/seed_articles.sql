-- Article Seeder: Creates sample articles with various categories for topic recommendations
-- Run this script manually after having at least one user in the database

-- First, let's get a user ID to use as author
-- If you have a specific user, replace the subquery with the actual UUID

DO $$
DECLARE
    author_id UUID;
    article_id UUID;
BEGIN
    -- Get the first available user as author
    SELECT id INTO author_id FROM users LIMIT 1;
    
    IF author_id IS NULL THEN
        RAISE EXCEPTION 'No users found. Please create a user first.';
    END IF;

    -- Technology Articles
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'The Future of AI in Software Development', 
     'How Machine Learning is Transforming the Way We Write Code',
     'Artificial Intelligence is revolutionizing software development. From code completion to automated testing, AI tools are becoming indispensable. In this article, we explore the latest trends and what they mean for developers...',
     '<p>Artificial Intelligence is revolutionizing software development. From code completion to automated testing, AI tools are becoming indispensable. In this article, we explore the latest trends and what they mean for developers...</p>',
     'Explore how AI is changing the landscape of software development and what developers need to know.',
     author_id, 'published', false, 'future-of-ai-software-development',
     ARRAY['AI', 'Machine Learning', 'Programming', 'Future Tech'],
     ARRAY['Technology'],
     8, 245, 1200, NOW() - INTERVAL '2 days', NOW() - INTERVAL '3 days', NOW()),
     
    (uuid_generate_v4(), 'Building Scalable Web Applications with Rust',
     'A Practical Guide to High-Performance Backend Development',
     'Rust has emerged as a powerful language for building fast, reliable, and memory-safe applications. This guide walks you through creating a scalable web application using Axum and SQLx...',
     '<p>Rust has emerged as a powerful language for building fast, reliable, and memory-safe applications. This guide walks you through creating a scalable web application using Axum and SQLx...</p>',
     'Learn how to build high-performance web applications using Rust, Axum, and SQLx.',
     author_id, 'published', false, 'scalable-web-apps-rust',
     ARRAY['Rust', 'Web Development', 'Backend', 'Performance'],
     ARRAY['Technology'],
     12, 389, 2100, NOW() - INTERVAL '5 days', NOW() - INTERVAL '6 days', NOW()),

    (uuid_generate_v4(), 'React 19: What New Features to Expect',
     'A Deep Dive into the Latest React Updates',
     'React 19 brings exciting new features including improved server components, better hydration, and enhanced developer experience. Let us explore what this means for your projects...',
     '<p>React 19 brings exciting new features including improved server components, better hydration, and enhanced developer experience. Let us explore what this means for your projects...</p>',
     'Discover the new features in React 19 and how they will improve your development workflow.',
     author_id, 'published', false, 'react-19-new-features',
     ARRAY['React', 'JavaScript', 'Frontend', 'Web Development'],
     ARRAY['Technology'],
     10, 567, 3400, NOW() - INTERVAL '1 day', NOW() - INTERVAL '2 days', NOW());

    -- Business Articles
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'The Art of Startup Fundraising',
     'Essential Strategies for Securing Investment in 2024',
     'Raising capital for your startup can be challenging. This comprehensive guide covers everything from pitch deck creation to negotiating term sheets with investors...',
     '<p>Raising capital for your startup can be challenging. This comprehensive guide covers everything from pitch deck creation to negotiating term sheets with investors...</p>',
     'Master the art of fundraising with proven strategies for attracting investors.',
     author_id, 'published', false, 'art-of-startup-fundraising',
     ARRAY['Startup', 'Investment', 'Fundraising', 'Entrepreneurship'],
     ARRAY['Business'],
     15, 423, 2800, NOW() - INTERVAL '4 days', NOW() - INTERVAL '5 days', NOW()),
     
    (uuid_generate_v4(), 'Remote Work: Building a Productive Team Culture',
     'Lessons from Leading Distributed Companies',
     'Remote work is here to stay. Learn how successful companies maintain culture, collaboration, and productivity across distributed teams with practical tips and tools...',
     '<p>Remote work is here to stay. Learn how successful companies maintain culture, collaboration, and productivity across distributed teams with practical tips and tools...</p>',
     'Build a thriving remote team culture with insights from successful distributed companies.',
     author_id, 'published', false, 'remote-work-team-culture',
     ARRAY['Remote Work', 'Team Culture', 'Productivity', 'Leadership'],
     ARRAY['Business'],
     11, 312, 1900, NOW() - INTERVAL '3 days', NOW() - INTERVAL '4 days', NOW());

    -- Health & Wellness Articles  
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'The Science of Sleep Optimization',
     'Evidence-Based Strategies for Better Rest',
     'Quality sleep is crucial for cognitive function, health, and productivity. Discover the latest research on sleep science and practical techniques for improving your sleep quality...',
     '<p>Quality sleep is crucial for cognitive function, health, and productivity. Discover the latest research on sleep science and practical techniques for improving your sleep quality...</p>',
     'Improve your sleep quality with evidence-based techniques backed by science.',
     author_id, 'published', false, 'science-of-sleep-optimization',
     ARRAY['Sleep', 'Health', 'Wellness', 'Productivity'],
     ARRAY['Health'],
     9, 654, 4200, NOW() - INTERVAL '1 day', NOW() - INTERVAL '2 days', NOW()),
     
    (uuid_generate_v4(), 'Mindfulness for Busy Professionals',
     'Simple Practices That Transform Your Workday',
     'Discover how busy executives and professionals incorporate mindfulness into their hectic schedules. These practical techniques can reduce stress and improve focus in just minutes a day...',
     '<p>Discover how busy executives and professionals incorporate mindfulness into their hectic schedules. These practical techniques can reduce stress and improve focus in just minutes a day...</p>',
     'Transform your workday with simple mindfulness practices designed for busy professionals.',
     author_id, 'published', false, 'mindfulness-busy-professionals',
     ARRAY['Mindfulness', 'Meditation', 'Stress Management', 'Work-Life Balance'],
     ARRAY['Health'],
     7, 445, 2600, NOW() - INTERVAL '6 days', NOW() - INTERVAL '7 days', NOW());

    -- Science Articles
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'Quantum Computing Explained Simply',
     'Understanding the Technology That Will Change Everything',
     'Quantum computing promises to revolutionize everything from drug discovery to cryptography. This article breaks down complex quantum concepts into understandable terms...',
     '<p>Quantum computing promises to revolutionize everything from drug discovery to cryptography. This article breaks down complex quantum concepts into understandable terms...</p>',
     'Understand quantum computing without the jargon with this accessible explanation.',
     author_id, 'published', false, 'quantum-computing-explained',
     ARRAY['Quantum Computing', 'Physics', 'Future Tech', 'Innovation'],
     ARRAY['Science'],
     13, 523, 3100, NOW() - INTERVAL '2 days', NOW() - INTERVAL '3 days', NOW()),
     
    (uuid_generate_v4(), 'The Mystery of Dark Matter',
     'What We Know and What Remains Unknown',
     'Dark matter makes up about 27% of the universe, yet we cannot see it. Explore what scientists have discovered about this mysterious substance and the ongoing research to understand it...',
     '<p>Dark matter makes up about 27% of the universe, yet we cannot see it. Explore what scientists have discovered about this mysterious substance and the ongoing research to understand it...</p>',
     'Dive into the mystery of dark matter and the scientific quest to understand it.',
     author_id, 'published', false, 'mystery-of-dark-matter',
     ARRAY['Dark Matter', 'Astronomy', 'Physics', 'Universe'],
     ARRAY['Science'],
     11, 387, 2400, NOW() - INTERVAL '5 days', NOW() - INTERVAL '6 days', NOW());

    -- Education Articles
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'Learning to Code: A Complete Roadmap',
     'From Zero to Full-Stack Developer in 12 Months',
     'Starting your coding journey can be overwhelming. This comprehensive roadmap guides you through learning programming from scratch, covering languages, frameworks, and best practices...',
     '<p>Starting your coding journey can be overwhelming. This comprehensive roadmap guides you through learning programming from scratch, covering languages, frameworks, and best practices...</p>',
     'Follow this complete roadmap to become a full-stack developer in one year.',
     author_id, 'published', false, 'learning-to-code-roadmap',
     ARRAY['Coding', 'Programming', 'Career', 'Learning'],
     ARRAY['Education'],
     16, 892, 5600, NOW() - INTERVAL '1 day', NOW() - INTERVAL '2 days', NOW()),

    (uuid_generate_v4(), 'The Future of Online Education',
     'How Technology is Reshaping How We Learn',
     'Online education has evolved beyond simple video courses. Explore the latest innovations in EdTech, from AI tutors to immersive VR classrooms...',
     '<p>Online education has evolved beyond simple video courses. Explore the latest innovations in EdTech, from AI tutors to immersive VR classrooms...</p>',
     'Discover how technology is transforming online education with AI and VR.',
     author_id, 'published', false, 'future-online-education',
     ARRAY['Education', 'EdTech', 'Online Learning', 'AI'],
     ARRAY['Education'],
     10, 345, 2100, NOW() - INTERVAL '4 days', NOW() - INTERVAL '5 days', NOW());

    -- Travel Articles
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'Hidden Gems of Southeast Asia',
     'Off-the-Beaten-Path Destinations You Need to Visit',
     'Beyond the tourist crowds lie incredible destinations waiting to be discovered. From secret beaches in Vietnam to ancient temples in Myanmar, explore Southeast Asia like a local...',
     '<p>Beyond the tourist crowds lie incredible destinations waiting to be discovered. From secret beaches in Vietnam to ancient temples in Myanmar, explore Southeast Asia like a local...</p>',
     'Discover hidden travel gems in Southeast Asia beyond the usual tourist spots.',
     author_id, 'published', false, 'hidden-gems-southeast-asia',
     ARRAY['Travel', 'Southeast Asia', 'Adventure', 'Culture'],
     ARRAY['Travel'],
     12, 678, 4100, NOW() - INTERVAL '2 days', NOW() - INTERVAL '3 days', NOW()),
     
    (uuid_generate_v4(), 'Digital Nomad Guide: Working While Traveling',
     'Everything You Need to Know About the Nomad Lifestyle',
     'Living as a digital nomad combines the freedom of travel with remote work. Learn about the best destinations, essential gear, and practical tips for living the nomad life...',
     '<p>Living as a digital nomad combines the freedom of travel with remote work. Learn about the best destinations, essential gear, and practical tips for living the nomad life...</p>',
     'Your complete guide to becoming a digital nomad and working while traveling.',
     author_id, 'published', false, 'digital-nomad-guide',
     ARRAY['Digital Nomad', 'Remote Work', 'Travel', 'Lifestyle'],
     ARRAY['Travel'],
     14, 534, 3300, NOW() - INTERVAL '3 days', NOW() - INTERVAL '4 days', NOW());

    -- Entertainment Articles
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'The Rise of Indie Games',
     'How Small Studios Are Changing the Gaming Industry',
     'Independent game developers are creating some of the most innovative and beloved games of our time. Explore the indie game revolution and the titles that are defining this era...',
     '<p>Independent game developers are creating some of the most innovative and beloved games of our time. Explore the indie game revolution and the titles that are defining this era...</p>',
     'Explore how indie game developers are revolutionizing the gaming industry.',
     author_id, 'published', false, 'rise-of-indie-games',
     ARRAY['Gaming', 'Indie Games', 'Game Development', 'Entertainment'],
     ARRAY['Entertainment'],
     9, 456, 2800, NOW() - INTERVAL '4 days', NOW() - INTERVAL '5 days', NOW()),
     
    (uuid_generate_v4(), 'Streaming Wars: The Battle for Your Screen',
     'Comparing Netflix, Disney+, HBO Max, and More',
     'With dozens of streaming services competing for attention, choosing where to spend your money can be challenging. This comprehensive comparison helps you decide...',
     '<p>With dozens of streaming services competing for attention, choosing where to spend your money can be challenging. This comprehensive comparison helps you decide...</p>',
     'Navigate the streaming landscape with this comprehensive comparison guide.',
     author_id, 'published', false, 'streaming-wars-comparison',
     ARRAY['Streaming', 'Netflix', 'Entertainment', 'TV Shows'],
     ARRAY['Entertainment'],
     11, 623, 3900, NOW() - INTERVAL '1 day', NOW() - INTERVAL '2 days', NOW());

    -- Food Articles
    INSERT INTO articles (id, title, subtitle, content, content_html, excerpt, author_id, status, is_member_only, slug, tags, categories, reading_time_minutes, claps_count, views_count, published_at, created_at, updated_at)
    VALUES 
    (uuid_generate_v4(), 'Plant-Based Cooking for Beginners',
     'Delicious Recipes That Will Make You Love Vegetables',
     'Transitioning to plant-based eating does not mean sacrificing flavor. Discover easy recipes and cooking techniques that make vegetables the star of every meal...',
     '<p>Transitioning to plant-based eating does not mean sacrificing flavor. Discover easy recipes and cooking techniques that make vegetables the star of every meal...</p>',
     'Start your plant-based journey with these delicious and easy recipes.',
     author_id, 'published', false, 'plant-based-cooking-beginners',
     ARRAY['Cooking', 'Plant-Based', 'Recipes', 'Healthy Eating'],
     ARRAY['Food'],
     8, 512, 3200, NOW() - INTERVAL '3 days', NOW() - INTERVAL '4 days', NOW()),
     
    (uuid_generate_v4(), 'The Perfect Home Coffee Setup',
     'Brew Cafe-Quality Coffee in Your Kitchen',
     'You do not need expensive equipment to make great coffee at home. Learn about the essential tools, techniques, and beans that will elevate your morning brew...',
     '<p>You do not need expensive equipment to make great coffee at home. Learn about the essential tools, techniques, and beans that will elevate your morning brew...</p>',
     'Create the perfect home coffee setup for cafe-quality brews every morning.',
     author_id, 'published', false, 'perfect-home-coffee-setup',
     ARRAY['Coffee', 'Brewing', 'Home Kitchen', 'Lifestyle'],
     ARRAY['Food'],
     7, 445, 2700, NOW() - INTERVAL '5 days', NOW() - INTERVAL '6 days', NOW());

    RAISE NOTICE 'Successfully seeded 16 articles across 8 categories!';
    
END $$;
