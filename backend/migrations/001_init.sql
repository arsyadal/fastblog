-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create custom types
CREATE TYPE user_type AS ENUM ('free', 'member', 'writer', 'publication');
CREATE TYPE article_status AS ENUM ('draft', 'published', 'unlisted', 'archived');
CREATE TYPE publication_role AS ENUM ('owner', 'editor', 'writer');
CREATE TYPE submission_status AS ENUM ('pending', 'approved', 'rejected', 'revision_requested');

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,
    display_name VARCHAR(100),
    bio TEXT,
    avatar_url TEXT,
    password_hash VARCHAR(255) NOT NULL,
    user_type user_type NOT NULL DEFAULT 'free',
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_banned BOOLEAN NOT NULL DEFAULT FALSE,
    membership_expires_at TIMESTAMPTZ,
    followers_count INTEGER NOT NULL DEFAULT 0,
    following_count INTEGER NOT NULL DEFAULT 0,
    articles_count INTEGER NOT NULL DEFAULT 0,
    total_claps_received BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Publications table
CREATE TABLE publications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    logo_url TEXT,
    banner_url TEXT,
    website_url TEXT,
    custom_domain VARCHAR(255) UNIQUE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    slug VARCHAR(50) UNIQUE NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    is_accepting_submissions BOOLEAN NOT NULL DEFAULT TRUE,
    followers_count INTEGER NOT NULL DEFAULT 0,
    articles_count INTEGER NOT NULL DEFAULT 0,
    writers_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Publication members table
CREATE TABLE publication_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    publication_id UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role publication_role NOT NULL DEFAULT 'writer',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(publication_id, user_id)
);

-- Articles table
CREATE TABLE articles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(200) NOT NULL,
    subtitle VARCHAR(300),
    content TEXT NOT NULL,
    content_html TEXT NOT NULL,
    excerpt TEXT,
    featured_image_url TEXT,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    publication_id UUID REFERENCES publications(id) ON DELETE SET NULL,
    status article_status NOT NULL DEFAULT 'draft',
    is_member_only BOOLEAN NOT NULL DEFAULT FALSE,
    is_featured BOOLEAN NOT NULL DEFAULT FALSE,
    paywall_position INTEGER,
    slug VARCHAR(255) NOT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}',
    reading_time_minutes INTEGER NOT NULL DEFAULT 1,
    claps_count BIGINT NOT NULL DEFAULT 0,
    comments_count INTEGER NOT NULL DEFAULT 0,
    bookmarks_count INTEGER NOT NULL DEFAULT 0,
    views_count BIGINT NOT NULL DEFAULT 0,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(author_id, slug)
);

-- User follows table
CREATE TABLE user_follows (
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (follower_id, following_id),
    CHECK (follower_id != following_id)
);

-- Publication follows table
CREATE TABLE publication_follows (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    publication_id UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, publication_id)
);

-- Claps table (Medium's signature feature)
CREATE TABLE claps (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    clap_count INTEGER NOT NULL DEFAULT 1 CHECK (clap_count >= 1 AND clap_count <= 50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, article_id)
);

-- Comments table
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    content_html TEXT NOT NULL,
    claps_count INTEGER NOT NULL DEFAULT 0,
    replies_count INTEGER NOT NULL DEFAULT 0,
    is_author_reply BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Bookmarks table
CREATE TABLE bookmarks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, article_id)
);

-- Highlights table (text selections with notes)
CREATE TABLE highlights (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    selected_text TEXT NOT NULL,
    note TEXT,
    start_position INTEGER NOT NULL,
    end_position INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (start_position < end_position)
);

-- Article views table (for analytics)
CREATE TABLE article_views (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    ip_address INET NOT NULL,
    user_agent TEXT,
    reading_time_seconds INTEGER,
    scroll_percentage REAL CHECK (scroll_percentage >= 0 AND scroll_percentage <= 100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Reading lists table
CREATE TABLE reading_lists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    articles_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Reading list articles table
CREATE TABLE reading_list_articles (
    reading_list_id UUID NOT NULL REFERENCES reading_lists(id) ON DELETE CASCADE,
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (reading_list_id, article_id)
);

-- Publication submissions table
CREATE TABLE publication_submissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    publication_id UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    writer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status submission_status NOT NULL DEFAULT 'pending',
    editor_notes TEXT,
    reviewed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ
);

-- Create indexes for performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created_at ON users(created_at);

CREATE INDEX idx_articles_author_id ON articles(author_id);
CREATE INDEX idx_articles_publication_id ON articles(publication_id);
CREATE INDEX idx_articles_status ON articles(status);
CREATE INDEX idx_articles_published_at ON articles(published_at);
CREATE INDEX idx_articles_created_at ON articles(created_at);
CREATE INDEX idx_articles_slug ON articles(slug);
CREATE INDEX idx_articles_tags ON articles USING GIN(tags);
CREATE INDEX idx_articles_featured ON articles(is_featured) WHERE is_featured = TRUE;

CREATE INDEX idx_user_follows_follower_id ON user_follows(follower_id);
CREATE INDEX idx_user_follows_following_id ON user_follows(following_id);

CREATE INDEX idx_publication_follows_user_id ON publication_follows(user_id);
CREATE INDEX idx_publication_follows_publication_id ON publication_follows(publication_id);

CREATE INDEX idx_claps_user_id ON claps(user_id);
CREATE INDEX idx_claps_article_id ON claps(article_id);
CREATE INDEX idx_claps_created_at ON claps(created_at);

CREATE INDEX idx_comments_article_id ON comments(article_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);
CREATE INDEX idx_comments_parent_id ON comments(parent_id);
CREATE INDEX idx_comments_created_at ON comments(created_at);

CREATE INDEX idx_bookmarks_user_id ON bookmarks(user_id);
CREATE INDEX idx_bookmarks_article_id ON bookmarks(article_id);

CREATE INDEX idx_highlights_user_id ON highlights(user_id);
CREATE INDEX idx_highlights_article_id ON highlights(article_id);

CREATE INDEX idx_article_views_article_id ON article_views(article_id);
CREATE INDEX idx_article_views_user_id ON article_views(user_id);
CREATE INDEX idx_article_views_created_at ON article_views(created_at);

CREATE INDEX idx_reading_lists_user_id ON reading_lists(user_id);
CREATE INDEX idx_reading_list_articles_list_id ON reading_list_articles(reading_list_id);
CREATE INDEX idx_reading_list_articles_article_id ON reading_list_articles(article_id);

-- Create functions for updating counters
CREATE OR REPLACE FUNCTION update_user_followers_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE users SET followers_count = followers_count + 1 WHERE id = NEW.following_id;
        UPDATE users SET following_count = following_count + 1 WHERE id = NEW.follower_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE users SET followers_count = followers_count - 1 WHERE id = OLD.following_id;
        UPDATE users SET following_count = following_count - 1 WHERE id = OLD.follower_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_article_claps_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE articles SET claps_count = claps_count + NEW.clap_count WHERE id = NEW.article_id;
        UPDATE users SET total_claps_received = total_claps_received + NEW.clap_count WHERE id = (SELECT author_id FROM articles WHERE id = NEW.article_id);
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        UPDATE articles SET claps_count = claps_count + (NEW.clap_count - OLD.clap_count) WHERE id = NEW.article_id;
        UPDATE users SET total_claps_received = total_claps_received + (NEW.clap_count - OLD.clap_count) WHERE id = (SELECT author_id FROM articles WHERE id = NEW.article_id);
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE articles SET claps_count = claps_count - OLD.clap_count WHERE id = OLD.article_id;
        UPDATE users SET total_claps_received = total_claps_received - OLD.clap_count WHERE id = (SELECT author_id FROM articles WHERE id = OLD.article_id);
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_article_comments_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE articles SET comments_count = comments_count + 1 WHERE id = NEW.article_id;
        IF NEW.parent_id IS NOT NULL THEN
            UPDATE comments SET replies_count = replies_count + 1 WHERE id = NEW.parent_id;
        END IF;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE articles SET comments_count = comments_count - 1 WHERE id = OLD.article_id;
        IF OLD.parent_id IS NOT NULL THEN
            UPDATE comments SET replies_count = replies_count - 1 WHERE id = OLD.parent_id;
        END IF;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_article_bookmarks_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE articles SET bookmarks_count = bookmarks_count + 1 WHERE id = NEW.article_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE articles SET bookmarks_count = bookmarks_count - 1 WHERE id = OLD.article_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers
CREATE TRIGGER trigger_update_user_followers_count
    AFTER INSERT OR DELETE ON user_follows
    FOR EACH ROW EXECUTE FUNCTION update_user_followers_count();

CREATE TRIGGER trigger_update_article_claps_count
    AFTER INSERT OR UPDATE OR DELETE ON claps
    FOR EACH ROW EXECUTE FUNCTION update_article_claps_count();

CREATE TRIGGER trigger_update_article_comments_count
    AFTER INSERT OR DELETE ON comments
    FOR EACH ROW EXECUTE FUNCTION update_article_comments_count();

CREATE TRIGGER trigger_update_article_bookmarks_count
    AFTER INSERT OR DELETE ON bookmarks
    FOR EACH ROW EXECUTE FUNCTION update_article_bookmarks_count();
