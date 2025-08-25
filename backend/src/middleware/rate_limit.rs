use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::sleep;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<DashMap<String, Vec<Instant>>>,
    max_requests: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_duration: Duration) -> Self {
        let rate_limiter = Self {
            requests: Arc::new(DashMap::new()),
            max_requests,
            window_duration,
        };

        // Cleanup task to remove old entries
        let requests_clone = rate_limiter.requests.clone();
        let window_duration_clone = window_duration;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let now = Instant::now();
                
                requests_clone.retain(|_, timestamps| {
                    timestamps.retain(|&timestamp| now.duration_since(timestamp) < window_duration_clone);
                    !timestamps.is_empty()
                });
            }
        });

        rate_limiter
    }

    pub fn check_rate_limit(&self, identifier: &str) -> bool {
        let now = Instant::now();
        
        let mut entry = self.requests.entry(identifier.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests outside the window
        entry.retain(|&timestamp| now.duration_since(timestamp) < self.window_duration);
        
        if entry.len() >= self.max_requests {
            false
        } else {
            entry.push(now);
            true
        }
    }
}

pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Create a simple rate limiter: 100 requests per minute per IP
    static RATE_LIMITER: once_cell::sync::Lazy<RateLimiter> = once_cell::sync::Lazy::new(|| {
        RateLimiter::new(100, Duration::from_secs(60))
    });

    let client_ip = addr.ip().to_string();
    
    if !RATE_LIMITER.check_rate_limit(&client_ip) {
        tracing::warn!("Rate limit exceeded for IP: {}", client_ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

// Stricter rate limiting for authentication endpoints
pub async fn auth_rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 5 auth attempts per minute per IP
    static AUTH_RATE_LIMITER: once_cell::sync::Lazy<RateLimiter> = once_cell::sync::Lazy::new(|| {
        RateLimiter::new(5, Duration::from_secs(60))
    });

    let client_ip = addr.ip().to_string();
    
    if !AUTH_RATE_LIMITER.check_rate_limit(&client_ip) {
        tracing::warn!("Auth rate limit exceeded for IP: {}", client_ip);
        
        // Add a small delay to slow down brute force attacks
        sleep(Duration::from_millis(1000)).await;
        
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}
