use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{AppState, services::search::SearchService};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(search))
        .route("/articles", get(search_articles))
        .route("/users", get(search_users))
        .route("/publications", get(search_publications))
        .route("/tags", get(search_tags))
        .route("/suggestions", get(get_search_suggestions))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<SearchSortBy>,
    pub filter: Option<SearchFilter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SearchSortBy {
    #[serde(rename = "relevance")]
    Relevance,
    #[serde(rename = "recent")]
    Recent,
    #[serde(rename = "popular")]
    Popular,
    #[serde(rename = "claps")]
    Claps,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SearchFilter {
    #[serde(rename = "articles")]
    Articles,
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "publications")]
    Publications,
    #[serde(rename = "tags")]
    Tags,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub total_results: i64,
    pub results: SearchResults,
    pub suggestions: Vec<String>,
    pub filters: SearchFilters,
}

#[derive(Debug, Serialize)]
pub struct SearchResults {
    pub articles: Vec<SearchArticleResult>,
    pub users: Vec<SearchUserResult>,
    pub publications: Vec<SearchPublicationResult>,
    pub tags: Vec<SearchTagResult>,
}

#[derive(Debug, Serialize)]
pub struct SearchArticleResult {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub excerpt: String,
    pub slug: Option<String>,
    pub author: SearchAuthorResult,
    pub publication: Option<SearchPublicationResult>,
    pub tags: Vec<String>,
    pub claps_count: i64,
    pub reading_time_minutes: i32,
    pub published_at: String,
    pub relevance_score: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchUserResult {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub followers_count: i32,
    pub articles_count: i32,
    pub is_verified: bool,
    pub relevance_score: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchPublicationResult {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub followers_count: i32,
    pub articles_count: i32,
    pub is_verified: bool,
    pub relevance_score: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchTagResult {
    pub name: String,
    pub articles_count: i64,
    pub followers_count: i32,
    pub relevance_score: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchAuthorResult {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SearchFilters {
    pub available_tags: Vec<String>,
    pub date_ranges: Vec<String>,
    pub reading_time_ranges: Vec<String>,
}

async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if query.q.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Search query cannot be empty"})),
        ));
    }

    let search_service = SearchService::new(state.db.pool.clone());
    
    match search_service.global_search(&query).await {
        Ok(results) => Ok(Json(serde_json::to_value(results).unwrap())),
        Err(e) => {
            tracing::error!("Search error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Search failed"})),
            ))
        }
    }
}

async fn search_articles(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if query.q.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Search query cannot be empty"})),
        ));
    }

    let search_service = SearchService::new(state.db.pool.clone());
    
    match search_service.search_articles(&query).await {
        Ok(results) => Ok(Json(json!({
            "query": query.q,
            "results": results,
            "total_results": results.len()
        }))),
        Err(e) => {
            tracing::error!("Article search error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Article search failed"})),
            ))
        }
    }
}

async fn search_users(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if query.q.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Search query cannot be empty"})),
        ));
    }

    let search_service = SearchService::new(state.db.pool.clone());
    
    match search_service.search_users(&query).await {
        Ok(results) => Ok(Json(json!({
            "query": query.q,
            "results": results,
            "total_results": results.len()
        }))),
        Err(e) => {
            tracing::error!("User search error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "User search failed"})),
            ))
        }
    }
}

async fn search_publications(
    State(_state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // TODO: Implement publication search
    Ok(Json(json!({
        "message": "Publication search not implemented yet",
        "query": query.q
    })))
}

async fn search_tags(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if query.q.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Search query cannot be empty"})),
        ));
    }

    let search_service = SearchService::new(state.db.pool.clone());
    
    match search_service.search_tags(&query).await {
        Ok(results) => Ok(Json(json!({
            "query": query.q,
            "results": results,
            "total_results": results.len()
        }))),
        Err(e) => {
            tracing::error!("Tag search error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Tag search failed"})),
            ))
        }
    }
}

async fn get_search_suggestions(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if query.q.trim().is_empty() {
        return Ok(Json(json!({
            "query": query.q,
            "suggestions": []
        })));
    }

    let search_service = SearchService::new(state.db.pool.clone());
    
    match search_service.get_search_suggestions(&query.q).await {
        Ok(suggestions) => Ok(Json(json!({
            "query": query.q,
            "suggestions": suggestions
        }))),
        Err(e) => {
            tracing::error!("Search suggestions error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to get search suggestions"})),
            ))
        }
    }
}
