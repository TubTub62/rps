use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:admin@localhost/as_server")
        .await
        .expect("Connection to Database Server failed");


    let sql : String = "SELECT * FROM alarm_share.alarm".to_string();
    let row = sqlx::query(&sql)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("Got: {:?}", row);

}
