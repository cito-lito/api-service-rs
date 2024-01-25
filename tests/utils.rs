use sqlx::PgPool;

pub async fn get_test_pool_db() -> PgPool {
    let test_db_url = "postgres://postgres:postgres@localhost:5432/test_db";
    PgPool::connect(test_db_url)
        .await
        .expect("Failed to create pool.")
}

pub async fn truncate_all_tables(pool: &PgPool) {
    sqlx::query("DO $do$ DECLARE r RECORD; BEGIN FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = current_schema()) LOOP EXECUTE 'TRUNCATE TABLE ' || quote_ident(r.tablename) || ' RESTART IDENTITY CASCADE'; END LOOP; END $do$;")
        .execute(pool)
        .await
        .expect("Failed to truncate all tables.");
}
