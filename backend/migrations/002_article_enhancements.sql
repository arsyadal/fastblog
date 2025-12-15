-- Add new fields to articles table for enhanced features
ALTER TABLE articles 
ADD COLUMN IF NOT EXISTS categories TEXT[] DEFAULT '{}',
ADD COLUMN IF NOT EXISTS reads_count BIGINT NOT NULL DEFAULT 0,
ADD COLUMN IF NOT EXISTS last_auto_save TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS auto_save_version INTEGER NOT NULL DEFAULT 0;

-- Create index for better performance on categories and tags
CREATE INDEX IF NOT EXISTS idx_articles_categories ON articles USING GIN(categories);
CREATE INDEX IF NOT EXISTS idx_articles_tags ON articles USING GIN(tags);

-- Create index for featured articles
CREATE INDEX IF NOT EXISTS idx_articles_featured ON articles(is_featured) WHERE is_featured = true;

-- Create index for reading time
CREATE INDEX IF NOT EXISTS idx_articles_reading_time ON articles(reading_time_minutes);

-- Create index for engagement metrics
CREATE INDEX IF NOT EXISTS idx_articles_engagement ON articles(views_count, reads_count, claps_count);

-- Add constraint to ensure auto_save_version is non-negative (only if not exists)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'check_auto_save_version' 
        AND conrelid = 'articles'::regclass
    ) THEN
        ALTER TABLE articles ADD CONSTRAINT check_auto_save_version CHECK (auto_save_version >= 0);
    END IF;
END $$;

-- Add constraint to ensure reads_count is non-negative (only if not exists)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint 
        WHERE conname = 'check_reads_count' 
        AND conrelid = 'articles'::regclass
    ) THEN
        ALTER TABLE articles ADD CONSTRAINT check_reads_count CHECK (reads_count >= 0);
    END IF;
END $$;

-- Create view for article statistics
CREATE OR REPLACE VIEW article_stats_view AS
SELECT 
    id,
    title,
    author_id,
    views_count,
    reads_count,
    claps_count,
    comments_count,
    bookmarks_count,
    reading_time_minutes,
    CASE 
        WHEN views_count > 0 THEN ROUND((reads_count::numeric / views_count::numeric) * 100, 2)
        ELSE 0 
    END as engagement_rate,
    published_at,
    created_at,
    updated_at
FROM articles 
WHERE status = 'published';

-- Create view for author statistics
CREATE OR REPLACE VIEW author_stats_view AS
SELECT 
    author_id,
    COUNT(*) as total_articles,
    SUM(views_count) as total_views,
    SUM(reads_count) as total_reads,
    SUM(claps_count) as total_claps,
    SUM(comments_count) as total_comments,
    SUM(bookmarks_count) as total_bookmarks,
    ROUND(AVG(reading_time_minutes), 1) as average_reading_time,
    CASE 
        WHEN SUM(views_count) > 0 THEN ROUND((SUM(reads_count)::numeric / SUM(views_count)::numeric) * 100, 2)
        ELSE 0 
    END as average_engagement_rate
FROM articles 
WHERE status = 'published'
GROUP BY author_id;
