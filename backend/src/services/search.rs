use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::models::{Article, User};
use crate::handlers::search::{
    SearchQuery, SearchResponse, SearchResults, SearchArticleResult, SearchUserResult, 
    SearchTagResult, SearchAuthorResult, SearchFilters, SearchSortBy
};

pub struct SearchService {
    pool: PgPool,
}

impl SearchService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Global search across all content types
    pub async fn global_search(&self, query: &SearchQuery) -> Result<SearchResponse, sqlx::Error> {
        let articles = self.search_articles_internal(query).await?;
        let users = self.search_users_internal(query).await?;
        let tags = self.search_tags_internal(query).await?;

        let total_results = articles.len() as i64 + users.len() as i64 + tags.len() as i64;

        Ok(SearchResponse {
            query: query.q.clone(),
            total_results,
            results: SearchResults {
                articles,
                users,
                publications: vec![], // TODO: Implement when publications are ready
                tags,
            },
            suggestions: self.get_search_suggestions(&query.q).await?,
            filters: SearchFilters {
                available_tags: self.get_popular_tags().await?,
                date_ranges: vec![
                    "Last week".to_string(),
                    "Last month".to_string(),
                    "Last year".to_string(),
                ],
                reading_time_ranges: vec![
                    "Under 5 min".to_string(),
                    "5-10 min".to_string(),
                    "10+ min".to_string(),
                ],
            },
        })
    }

    /// Search articles by title, subtitle, content, and tags
    pub async fn search_articles(&self, query: &SearchQuery) -> Result<Vec<SearchArticleResult>, sqlx::Error> {
        self.search_articles_internal(query).await
    }

    async fn search_articles_internal(&self, query: &SearchQuery) -> Result<Vec<SearchArticleResult>, sqlx::Error> {
        let limit = query.limit.unwrap_or(20);
        let offset = ((query.page.unwrap_or(1) - 1) * limit).max(0);
        let search_term = format!("%{}%", query.q.to_lowercase());

        let sort_clause = match query.sort.as_ref().unwrap_or(&SearchSortBy::Relevance) {
            SearchSortBy::Relevance => "ORDER BY relevance_score DESC",
            SearchSortBy::Recent => "ORDER BY a.published_at DESC NULLS LAST",
            SearchSortBy::Popular => "ORDER BY COALESCE(a.views_count, 0) DESC",
            SearchSortBy::Claps => "ORDER BY COALESCE(a.claps_count, 0) DESC",
        };

        let sql = format!(r#"
            WITH article_search AS (
                SELECT 
                    a.id,
                    a.title,
                    a.subtitle,
                    a.content,
                    a.slug,
                    a.author_id,
                    a.claps_count,
                    a.views_count,
                    a.reading_time_minutes,
                    a.published_at,
                    a.created_at,
                    u.username as author_username,
                    u.display_name as author_display_name,
                    u.bio as author_bio,
                    u.avatar_url as author_avatar_url,
                    -- Enhanced relevance scoring for better keyword matching
                    (
                        -- Title matches (highest priority)
                        CASE WHEN LOWER(a.title) LIKE $1 THEN 15 ELSE 0 END +
                        CASE WHEN LOWER(a.title) LIKE $2 THEN 12 ELSE 0 END +
                        CASE WHEN LOWER(a.title) LIKE $3 THEN 10 ELSE 0 END +
                        
                        -- Subtitle matches
                        CASE WHEN LOWER(a.subtitle) LIKE $1 THEN 8 ELSE 0 END +
                        CASE WHEN LOWER(a.subtitle) LIKE $2 THEN 6 ELSE 0 END +
                        CASE WHEN LOWER(a.subtitle) LIKE $3 THEN 5 ELSE 0 END +
                        
                        -- Content matches (word boundaries and partial)
                        CASE WHEN LOWER(a.content) LIKE $1 THEN 6 ELSE 0 END +
                        CASE WHEN LOWER(a.content) LIKE $2 THEN 4 ELSE 0 END +
                        CASE WHEN LOWER(a.content) LIKE $3 THEN 2 ELSE 0 END +
                        
                        -- Author matches
                        CASE WHEN LOWER(u.username) LIKE $1 THEN 5 ELSE 0 END +
                        CASE WHEN LOWER(u.display_name) LIKE $1 THEN 5 ELSE 0 END +
                        CASE WHEN LOWER(u.bio) LIKE $1 THEN 3 ELSE 0 END +
                        
                        -- Bonus for multiple word matches
                        CASE WHEN LOWER(a.title) LIKE $4 THEN 3 ELSE 0 END +
                        CASE WHEN LOWER(a.content) LIKE $4 THEN 2 ELSE 0 END
                    )::float as relevance_score
                FROM articles a
                INNER JOIN users u ON a.author_id = u.id
                WHERE 
                    a.status = 'published'
                    AND (
                        -- Multiple search patterns for better matching
                        LOWER(a.title) LIKE $1 
                        OR LOWER(a.title) LIKE $2
                        OR LOWER(a.title) LIKE $3
                        OR LOWER(a.subtitle) LIKE $1 
                        OR LOWER(a.subtitle) LIKE $2
                        OR LOWER(a.subtitle) LIKE $3
                        OR LOWER(a.content) LIKE $1 
                        OR LOWER(a.content) LIKE $2
                        OR LOWER(a.content) LIKE $3
                        OR LOWER(u.username) LIKE $1 
                        OR LOWER(u.display_name) LIKE $1
                        OR LOWER(u.bio) LIKE $1
                        OR LOWER(a.content) LIKE $4
                    )
            )
            SELECT * FROM article_search
            WHERE relevance_score > 0
            {}
            LIMIT $5 OFFSET $6
        "#, sort_clause);

        // Create multiple search patterns for better keyword matching
        let exact_match = format!("%{}%", query.q.to_lowercase());
        let word_boundary = format!("% {} %", query.q.to_lowercase());
        let partial_start = format!("{}%", query.q.to_lowercase());
        let multi_word = format!("%{}%", query.q.split_whitespace().collect::<Vec<_>>().join("%"));
        
        let rows = sqlx::query(&sql)
            .bind(&exact_match)      // $1 - exact match
            .bind(&word_boundary)    // $2 - word boundary
            .bind(&partial_start)    // $3 - partial start
            .bind(&multi_word)       // $4 - multi-word
            .bind(limit)             // $5 - limit
            .bind(offset)            // $6 - offset
            .fetch_all(&self.pool)
            .await?;

        let mut results = Vec::new();
        for row in rows {
            let content: String = row.get("content");
            let excerpt = self.create_excerpt(&content, &query.q);
            
            results.push(SearchArticleResult {
                id: row.get::<Uuid, _>("id").to_string(),
                title: row.get("title"),
                subtitle: row.get("subtitle"),
                excerpt,
                slug: row.get("slug"),
                author: SearchAuthorResult {
                    id: row.get::<Uuid, _>("author_id").to_string(),
                    username: row.get("author_username"),
                    display_name: row.get("author_display_name"),
                    avatar_url: row.get("author_avatar_url"),
                },
                publication: None, // TODO: Add when publications are implemented
                tags: vec![], // TODO: Implement tags
                claps_count: row.get::<Option<i64>, _>("claps_count").unwrap_or(0),
                reading_time_minutes: row.get::<Option<i32>, _>("reading_time_minutes").unwrap_or(5),
                published_at: row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("published_at")
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
                relevance_score: row.get::<f64, _>("relevance_score") as f32,
            });
        }

        Ok(results)
    }

    /// Search users by username, display name, and bio
    pub async fn search_users(&self, query: &SearchQuery) -> Result<Vec<SearchUserResult>, sqlx::Error> {
        self.search_users_internal(query).await
    }

    async fn search_users_internal(&self, query: &SearchQuery) -> Result<Vec<SearchUserResult>, sqlx::Error> {
        let limit = query.limit.unwrap_or(20);
        let offset = ((query.page.unwrap_or(1) - 1) * limit).max(0);
        let search_term = format!("%{}%", query.q.to_lowercase());

        let sort_clause = match query.sort.as_ref().unwrap_or(&SearchSortBy::Relevance) {
            SearchSortBy::Relevance => "ORDER BY relevance_score DESC",
            SearchSortBy::Recent => "ORDER BY u.created_at DESC",
            SearchSortBy::Popular => "ORDER BY COALESCE(followers_count, 0) DESC",
            _ => "ORDER BY relevance_score DESC",
        };

        let sql = format!(r#"
            WITH user_search AS (
                SELECT 
                    u.id,
                    u.username,
                    u.display_name,
                    u.bio,
                    u.avatar_url,
                    u.is_verified,
                    u.created_at,
                    COUNT(DISTINCT f.follower_id) as followers_count,
                    COUNT(DISTINCT a.id) as articles_count,
                    -- Enhanced relevance scoring for better keyword matching
                    (
                        -- Username matches (highest priority)
                        CASE WHEN LOWER(u.username) LIKE $1 THEN 15 ELSE 0 END +
                        CASE WHEN LOWER(u.username) LIKE $2 THEN 12 ELSE 0 END +
                        CASE WHEN LOWER(u.username) LIKE $3 THEN 10 ELSE 0 END +
                        
                        -- Display name matches
                        CASE WHEN LOWER(u.display_name) LIKE $1 THEN 12 ELSE 0 END +
                        CASE WHEN LOWER(u.display_name) LIKE $2 THEN 10 ELSE 0 END +
                        CASE WHEN LOWER(u.display_name) LIKE $3 THEN 8 ELSE 0 END +
                        
                        -- Bio matches
                        CASE WHEN LOWER(u.bio) LIKE $1 THEN 8 ELSE 0 END +
                        CASE WHEN LOWER(u.bio) LIKE $2 THEN 6 ELSE 0 END +
                        CASE WHEN LOWER(u.bio) LIKE $3 THEN 4 ELSE 0 END +
                        
                        -- Bonus for multiple word matches
                        CASE WHEN LOWER(u.username) LIKE $4 THEN 3 ELSE 0 END +
                        CASE WHEN LOWER(u.display_name) LIKE $4 THEN 3 ELSE 0 END +
                        CASE WHEN LOWER(u.bio) LIKE $4 THEN 2 ELSE 0 END
                    )::float as relevance_score
                FROM users u
                LEFT JOIN user_follows f ON u.id = f.following_id
                LEFT JOIN articles a ON u.id = a.author_id AND a.status = 'published'
                WHERE 
                    -- Multiple search patterns for better matching
                    LOWER(u.username) LIKE $1 
                    OR LOWER(u.username) LIKE $2
                    OR LOWER(u.username) LIKE $3
                    OR LOWER(u.display_name) LIKE $1 
                    OR LOWER(u.display_name) LIKE $2
                    OR LOWER(u.display_name) LIKE $3
                    OR LOWER(u.bio) LIKE $1
                    OR LOWER(u.bio) LIKE $2
                    OR LOWER(u.bio) LIKE $3
                    OR LOWER(u.username) LIKE $4
                    OR LOWER(u.display_name) LIKE $4
                    OR LOWER(u.bio) LIKE $4
                GROUP BY u.id, u.username, u.display_name, u.bio, u.avatar_url, u.is_verified, u.created_at
            )
            SELECT * FROM user_search
            WHERE relevance_score > 0
            {}
            LIMIT $5 OFFSET $6
        "#, sort_clause);

        // Create multiple search patterns for better keyword matching
        let exact_match = format!("%{}%", query.q.to_lowercase());
        let word_boundary = format!("% {} %", query.q.to_lowercase());
        let partial_start = format!("{}%", query.q.to_lowercase());
        let multi_word = format!("%{}%", query.q.split_whitespace().collect::<Vec<_>>().join("%"));
        
        let rows = sqlx::query(&sql)
            .bind(&exact_match)      // $1 - exact match
            .bind(&word_boundary)    // $2 - word boundary
            .bind(&partial_start)    // $3 - partial start
            .bind(&multi_word)       // $4 - multi-word
            .bind(limit)             // $5 - limit
            .bind(offset)            // $6 - offset
            .fetch_all(&self.pool)
            .await?;

        let mut results = Vec::new();
        for row in rows {
            results.push(SearchUserResult {
                id: row.get::<Uuid, _>("id").to_string(),
                username: row.get("username"),
                display_name: row.get("display_name"),
                bio: row.get("bio"),
                avatar_url: row.get("avatar_url"),
                followers_count: row.get::<i64, _>("followers_count").min(i32::MAX as i64) as i32,
                articles_count: row.get::<i64, _>("articles_count").min(i32::MAX as i64) as i32,
                is_verified: row.get("is_verified"),
                relevance_score: row.get::<f64, _>("relevance_score") as f32,
            });
        }

        Ok(results)
    }

    /// Search tags (placeholder for future implementation)
    pub async fn search_tags(&self, query: &SearchQuery) -> Result<Vec<SearchTagResult>, sqlx::Error> {
        self.search_tags_internal(query).await
    }

    async fn search_tags_internal(&self, _query: &SearchQuery) -> Result<Vec<SearchTagResult>, sqlx::Error> {
        // TODO: Implement when tags table is created
        Ok(vec![])
    }

    /// Get search suggestions based on popular searches and content
    pub async fn get_search_suggestions(&self, query: &str) -> Result<Vec<String>, sqlx::Error> {
        let search_term = format!("%{}%", query.to_lowercase());
        
        let sql = r#"
            (
                SELECT DISTINCT title as suggestion
                FROM articles 
                WHERE LOWER(title) LIKE $1 AND status = 'published'
                LIMIT 5
            )
            UNION ALL
            (
                SELECT DISTINCT 
                    CASE 
                        WHEN display_name IS NOT NULL THEN display_name 
                        ELSE username 
                    END as suggestion
                FROM users 
                WHERE LOWER(username) LIKE $1 OR LOWER(display_name) LIKE $1
                LIMIT 5
            )
            ORDER BY suggestion ASC
            LIMIT 10
        "#;

        let rows = sqlx::query(sql)
            .bind(&search_term)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter()
            .map(|row| row.get::<String, _>("suggestion"))
            .collect())
    }

    /// Get popular tags for filters
    async fn get_popular_tags(&self) -> Result<Vec<String>, sqlx::Error> {
        // TODO: Implement when tags are implemented
        Ok(vec![
            "Technology".to_string(),
            "Programming".to_string(),
            "Design".to_string(),
            "Writing".to_string(),
            "Business".to_string(),
        ])
    }

    /// Create an excerpt from content highlighting the search term
    fn create_excerpt(&self, content: &str, search_term: &str) -> String {
        let content_lower = content.to_lowercase();
        let search_lower = search_term.to_lowercase();
        
        if let Some(pos) = content_lower.find(&search_lower) {
            let start = pos.saturating_sub(50);
            let end = (pos + search_term.len() + 100).min(content.len());
            
            let mut excerpt = content[start..end].to_string();
            if start > 0 {
                excerpt = format!("...{}", excerpt);
            }
            if end < content.len() {
                excerpt = format!("{}...", excerpt);
            }
            
            // Clean up any incomplete words at the beginning/end
            excerpt
        } else {
            // If search term not found, just take first 150 characters
            let end = 150.min(content.len());
            let mut excerpt = content[..end].to_string();
            if end < content.len() {
                excerpt = format!("{}...", excerpt);
            }
            excerpt
        }
    }
}
