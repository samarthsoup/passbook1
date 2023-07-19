use std::time::Instant;
use tokio_postgres::{NoTls, Client};

use crate::functions::{User, Transaction};

pub async fn database_connection() -> Client{
    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:postgres@localhost/postgres", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

pub async fn distinct_check(column_to_check:String, column_to_check_value:i32) -> i64 {
    let start = Instant::now();
    let client = database_connection().await;

    let distinct_check_query = format!("SELECT COUNT(*) FROM users WHERE {} = $1",column_to_check);

    let distinct_check = client.query_one(&distinct_check_query, &[&column_to_check_value]).await.unwrap();
    let count:i64 =  distinct_check.get(0);

    let duration = start.elapsed();
    println!("checking if a value is distinct in the database- time elapsed: {:?}", duration);

    count
}

pub async fn insert_into_users(user:User) -> User{
    let start = Instant::now();
    let client = database_connection().await;

    let insert_query = format!("INSERT INTO users (userid, name, balance) VALUES ($1, $2, $3)");

    client.execute(&insert_query, &[&user.userid, &user.name, &user.balance]).await.unwrap();

    let select_query = format!("SELECT * FROM users WHERE userid = $1");
    let row = client.query_one(&select_query, &[&user.userid]).await.unwrap();

    let user = User{
        userid: row.get(0),
        name: row.get(1),
        balance: row.get(2)
    };

    let duration = start.elapsed();
    println!("inserted a user into the users table- time elapsed: {:?}", duration);

    user
}

pub async fn insert_into_transactions(date:String, userid:i32, amount:f64, category:String){
    let start = Instant::now();
    let client = database_connection().await;

    let insert_query = format!("INSERT INTO transactions (date, userid, amount, category) VALUES ($1, $2, $3, $4) RETURNING id");
    let row = client.query_one(&insert_query, &[&date,&userid,&amount,&category]).await.unwrap();
    let transaction_id:i32 = row.get(0);

    println!("transaction ID {} inserted successfully.", transaction_id);

    let duration = start.elapsed();
    println!("inserted a transaction into the transaction table- time elapsed: {:?}", duration);
}

pub async fn select_from_users(userid:i32) -> User{
    let start = Instant::now();
    let client = database_connection().await;

    let select_query = format!("SELECT * FROM users WHERE userid = $1");
    let row = client.query_one(&select_query, &[&userid]).await.unwrap();

    let user = User{
        userid: row.get(0),
        name: row.get(1),
        balance: row.get(2)
    };

    let duration = start.elapsed();
    println!("selecting a user from the users table- time elapsed: {:?}", duration);

    user
}

pub async fn select_from_transactions(userid:i32) -> Vec<Transaction> {
    let start = Instant::now();
    let client = database_connection().await;

    let select_query = "
        SELECT date, userid, amount, category
        FROM transactions
        WHERE userid = $1
    ";

    let mut transactions: Vec<Transaction> = Vec::new();

    let rows = client.query(select_query, &[&userid]).await.unwrap();

    for row in rows {
        let transaction = Transaction {
            date: row.get(0),
            userid: row.get(1),
            amount: row.get(2),
            category: row.get(3)
        };

        transactions.push(transaction);
    }

    let duration = start.elapsed();
    println!("selected transactions from the transactions table- time elasped: {:?}", duration);

    transactions
}

pub async fn update(userid:i32, amount:f64){
    let start = Instant::now();
    let client = database_connection().await;

    let update_query = format!("UPDATE users SET balance = balance + $1 WHERE userid = $2");
    client.execute(&update_query, &[&amount, &userid]).await.unwrap();

    let duration = start.elapsed();
    println!("updated balance in users table- time elapsed: {:?}", duration);
}

pub async fn delete_row(userid:i32){
    let start = Instant::now();
    let client = database_connection().await;

    let delete_query = format!("DELETE FROM users WHERE userid = $1");

    client.execute(&delete_query, &[&userid]).await.unwrap();

    let duration = start.elapsed();
    println!("deleted user from table- time elapsed: {:?}", duration);
}