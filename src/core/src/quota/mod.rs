pub async fn is_quota_blocked(org_id: &str) -> bool {
    let redis_url = std::env::var("ZO_REDIS_URL")
        .or_else(|_| {
            std::env::var("REDIS_ADDR")
                .map(|addr| {
                    if addr.starts_with("redis://") {
                        addr
                    } else {
                        format!("redis://{}/", addr)
                    }
                })
        })
        .unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());

    let client = match redis::Client::open(redis_url.as_str()) {
        Ok(c) => c,
        Err(_) => return false, // Fail-open: jika Redis error/unreachable, biarkan log tetap masuk
    };
    let mut con = match client.get_async_connection().await {
        Ok(conn) => conn,
        Err(_) => return false,
    };
    let blocked: Result<bool, _> = redis::cmd("GET")
        .arg(format!("quota:blocked:{}", org_id))
        .query_async(&mut con)
        .await;

    blocked.unwrap_or(false)
}
