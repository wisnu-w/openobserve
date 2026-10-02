pub async fn is_quota_blocked(org_id: &str) -> bool {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(c) => c,
        Err(_) => return false, // Fail-open: jika Redis error, biarkan log tetap masuk
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
