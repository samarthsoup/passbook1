use tokio_postgres::{NoTls, Row};
use chrono::Utc;

use crate::functions::User;

pub async fn distinct_check(column_to_check:String, column_to_check_value:i32) -> i64 {
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let distinct_check_query = format!("SELECT COUNT(*) FROM users WHERE {} = $1",column_to_check);

    let distinct_check = client.query_one(&distinct_check_query, &[&column_to_check_value]).await.unwrap();
    let count:i64 =  distinct_check.get(0);

    count
}

pub async fn insert_into_users(user:User) -> Row{
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let insert_query = format!("INSERT INTO users (userid, name, balance) VALUES ($1, $2, $3)");

    client.execute(&insert_query, &[&user.userid, &user.name, &user.balance]).await.unwrap();

    let select_query = format!("SELECT * FROM users WHERE userid = $1");
    let row = client.query_one(&select_query, &[&user.userid]).await.unwrap();

    row
}

pub async fn insert_into_transactions(userid:i32, amount:f64, category:String){
    let current_date = Utc::now().naive_utc();
    let date = current_date.to_string();

    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let insert_query = format!("INSERT INTO transactions (date, userid, amount, category) VALUES ($1, $2, $3, $4) RETURNING id");
    let row = client.query_one(&insert_query, &[&date,&userid,&amount,&category]).await.unwrap();
    let transaction_id:i32 = row.get(0);

    println!("transaction ID {} inserted successfully.", transaction_id);
}

pub async fn select_from_users(userid:i32) -> Row{
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let select_query = format!("SELECT * FROM users WHERE userid = $1");
    let row = client.query_one(&select_query, &[&userid]).await.unwrap();

    row
}

pub async fn select_from_transactions(userid:i32) -> Vec<Row> {
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();
    tokio::spawn(async move {   
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let select_query = "
        SELECT date, amount, category
        FROM transactions
        WHERE userid = $1
    ";

    let rows = client.query(select_query, &[&userid]).await.unwrap();

    rows
}

pub async fn update(userid:i32, amount:f64){
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let update_query = format!("UPDATE users SET balance = balance + $1 WHERE userid = $2");
    client.execute(&update_query, &[&amount, &userid]).await.unwrap();

}

pub async fn delete_row(userid:i32){
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let delete_query = format!("DELETE FROM users WHERE userid = $1");

    client.execute(&delete_query, &[&userid]).await.unwrap();
}