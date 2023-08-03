use std::time::Instant;
use chrono::Utc;

use crate::functions::{User, Data, Transaction};

pub fn handle_signup_request(data: axum::extract::Json<Data>) -> User{
    let start = Instant::now();

    let input = Data {
        userid: data.userid.clone(),
        name: data.name.clone(),
        balance: None,
        amount: None,
        category: None
    };

    let userid = input.userid.expect("userid field empty").parse::<i32>().unwrap();
    let name = input.name.expect("name field empty");

    let user = User { 
        userid, 
        name, 
        balance: 0.0 
    };

    println!("http response payload: {:?}", data);

    let duration = start.elapsed();
    println!("payload put into a User struct- time elapsed: {:?}\n", duration);

    user
}

pub fn handle_login_form(data: axum::extract::Json<Data>) -> i32 {
    let input = Data {
        userid: data.userid.clone(),
        name: None,
        balance: None,
        amount: None,
        category: None
    };

    let userid = input.userid.expect("userid field empty").parse::<i32>().unwrap();

    println!("logging in with userid: {}\n", userid);
    userid
}

pub fn handle_transaction_form(data: axum::extract::Json<Data>) -> Transaction{
    let input = Data {
        userid: data.userid.clone(),
        name: None,
        balance: None,
        amount: data.amount.clone(),
        category: data.category.clone()
    };

    let current_date = Utc::now().naive_utc();
    let date = current_date.to_string();

    let userid = input.userid.expect("userid field empty").parse::<i32>().unwrap();
    let amount = input.amount.expect("amount field empty").parse::<f64>().unwrap();
    let category = input.category.expect("category field empty");

    let transaction = Transaction{
        date,
        userid,
        amount,
        category
    };

    println!("http payload is stored in Transaction struct");
    println!("transaction details- userid: {}, amount: {}, category: {}\n", transaction.userid, transaction.amount, transaction.category);

    transaction
}