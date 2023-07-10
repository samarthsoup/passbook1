use axum::{response::Html};
use minijinja::render;
use serde::{Deserialize, Serialize};
use std::time::{Instant};
use crate::database::{insert_into_users, select_from_users, update, delete_row, select_from_transactions, insert_into_transactions};

use crate::html::{
    DEPOSIT, FAILURE, HISTORY, HOME, LOGIN, LOGINACTIVITY, REMOVESUCCESS, SIGNUP,
    SIGNUPFAILURE, SIGNUPSUCCESS, SUCCESS, USERPAGE, WITHDRAW, LOGINFAILURE
};

pub use crate::database::{
    distinct_check
};

pub async fn home() -> Html<String> {
    let start = Instant::now();

    let r = render!(HOME);

    let duration = start.elapsed();
    println!("rendering HOME-time elapsed: {:?}", duration);

    Html(r)
}

pub async fn signup() -> Html<String> {
    let start = Instant::now();

    let n = render!(SIGNUP);

    let duration = start.elapsed();
    println!("rendering SIGNUP-time elapsed: {:?}", duration);

    Html(n)
}

#[derive(Deserialize)]
pub struct SignupInput {
    id: i32,
    name: String,
}

#[derive(Serialize)]
pub struct User{
    pub userid:i32,
    pub name:String,
    pub balance:f64
}

pub async fn signupactivity(form: axum::Form<SignupInput>) -> Html<String> {
    let start = Instant::now();
    let accno = form.id;
    let accname = &form.name;
    println!("user's input- account number: {}, name: {}", accno, accname);

    let user = User{
        userid: accno,
        name: accname.to_string(),
        balance: 0.0
    };

    let count = distinct_check( "userid".to_string(), user.userid).await;

    if count > 0 {
        let b = render!(SIGNUPFAILURE);
        return Html(b);
    }else {
        let row = insert_into_users(user).await;
        
        let inserted_userid: i32 = row.get(0);
        let inserted_name: &str = row.get(1);
        println!("successfully inserted- userid: {}, name: {}", inserted_userid, inserted_name);
    }

    let u = render!(SIGNUPSUCCESS);

    let duration = start.elapsed();
    println!("SIGNUP activity-time elapsed: {:?}", duration);

    Html(u)
}

pub async fn login() -> Html<String> {
    let start = Instant::now();

    let r = render!(LOGIN);

    let duration = start.elapsed();
    println!("rendering LOGIN-time elapsed: {:?}", duration);

    Html(r)
}

#[derive(Deserialize)]
pub struct LoginInput {
    id: i32,
}

pub async fn loginactivity(form: axum::Form<LoginInput>) -> Html<String>{
    let start = Instant::now();

    let userid = form.id;

    let count = distinct_check("userid".to_string(), userid).await;

    if count > 0 {
        let r = render!(LOGINACTIVITY, userid => userid);

        let duration = start.elapsed();
        println!("LOGIN activity(success)-time elapsed: {:?}", duration);

        return Html(r);
    }

    let b = render!(LOGINFAILURE, userid => userid);

    let duration = start.elapsed();
    println!("LOGIN activity(failure)-time elapsed: {:?}", duration);

    Html(b)
}

pub async fn userpage(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let row = select_from_users(userid).await;
    
    let user = User{
        userid: row.get(0),
        name: row.get(1),
        balance: row.get(2)
    };

    let r = render!(USERPAGE, user => user);

    let duration = start.elapsed();
    println!("rendering USERPAGE-time elapsed: {:?}", duration);

    Html(r)
}

#[derive(Debug,Serialize)]
pub struct Transaction {
    date: String,
    amount: f64,
    category: String
}

pub async fn history(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let rows = select_from_transactions(userid).await;

    let mut transactions: Vec<Transaction> = Vec::new();

    for row in rows {
        let transaction = Transaction {
            date: row.get(0),
            amount: row.get(1),
            category: row.get(2)
        };

        transactions.push(transaction);
    }

    let r = render!(HISTORY, transactions => transactions, userid => userid);

    let duration = start.elapsed();
    println!("rendering HISTORY-time elapsed: {:?}", duration);

    Html(r)
}

pub async fn deposit(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();
    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let row = select_from_users(userid).await;

    let user = User{
        userid: row.get(0),
        name: row.get(1),
        balance: row.get(2)
    };

    let q = render!(DEPOSIT, user => user);

    let duration = start.elapsed();
    println!("rendering DEPOSIT-time elapsed: {:?}", duration);

    Html(q)
}

#[derive(Deserialize)]
pub struct UserInput {
    id: i32, 
    amount: f64,
    category: String
}

pub async fn depositactivity(form: axum::Form<UserInput>) -> Html<String> {
    let start = Instant::now();

    let userid = form.id;
    let amount = form.amount;
    let category = &form.category;

    insert_into_transactions(userid, amount, category.to_string()).await;

    update(userid, amount).await;

    let row = select_from_users(userid).await;
    
    let user = User{
        userid: row.get(0),
        name: row.get(1),
        balance: row.get(2)
    };
    
    let y = render!(SUCCESS, user => user);

    println!("money added successfully");    

    let duration = start.elapsed();
    println!("DEPOSIT activity-time elapsed: {:?}", duration);

    Html(y)
}

pub async fn withdraw(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let row = select_from_users(userid).await;

    let user = User{
        userid: row.get(0),
        name: row.get(1),
        balance: row.get(2)
    };
    let s = render!(WITHDRAW, user => user);

    let duration = start.elapsed();
    println!("rendering WITHDRAW-time elapsed: {:?}", duration);

    Html(s)
}

pub async fn withdrawactivity(form: axum::Form<UserInput>) -> Html<String> {
    let start = Instant::now();

    let amount = form.amount;
    let userid = form.id;
    let category = &form.category;

    let row = select_from_users(userid).await;

    let user = User{
        userid: row.get(0),
        name: row.get(1),
        balance: row.get(2)
    };

    if user.balance < amount {
        let u = render!(FAILURE, user => user);

        let duration = start.elapsed();
        println!("WITHDRAW activity(failure)-time elapsed: {:?}", duration);

        return Html(u);
    } else {
        update(userid, -amount).await;
        insert_into_transactions(userid, -amount, category.to_string()).await;
    }
    
    let x = render!(SUCCESS, user => user);

    let duration = start.elapsed();
    println!("WITHDRAW activity(success)-time elapsed: {:?}", duration);

    Html(x)
}

pub async fn delete(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    delete_row(userid).await;

    let h = render!(REMOVESUCCESS);

    let duration = start.elapsed();
    println!("deleted a user's file-time elapsed: {:?}", duration);

    Html(h)
}