-- Add user follows table for social features
CREATE TABLE IF NOT EXISTS user_follows (
    follower_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    PRIMARY KEY (follower_id, following_id),
    
    -- Prevent users from following themselves
    CONSTRAINT no_self_follow CHECK (follower_id != following_id)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_user_follows_follower ON user_follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_user_follows_following ON user_follows(following_id);
CREATE INDEX IF NOT EXISTS idx_user_follows_created_at ON user_follows(created_at);
