use axum::response::Html;
use minijinja::render;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use chrono::Utc;

use crate::database::{insert_into_users, select_from_users, update, delete_row, select_from_transactions, insert_into_transactions, distinct_check};

use crate::html::{
    DEPOSIT, FAILURE, HISTORY, HOME, LOGIN, LOGINACTIVITY, REMOVESUCCESS, SIGNUP,
    SIGNUPFAILURE, SIGNUPSUCCESS, SUCCESS, USERPAGE, WITHDRAW, LOGINFAILURE
};

#[derive(Serialize)]
pub struct User{
    pub userid:i32,
    pub name:String,
    pub balance:f64
}

pub async fn home() -> Html<String> {
    let start = Instant::now();

    let r = render!(HOME);

    let duration = start.elapsed();
    println!("rendering HOME-time elapsed: {:?}", duration);

    Html(r)
}

//signup
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

fn handle_signup_form(form: axum::Form<SignupInput>) -> User{
    let user = User{
        userid: form.id,
        name: form.name.to_string(), 
        balance: 0.0
    };

    user
}

pub async fn signupactivity(form: axum::Form<SignupInput>) -> Html<String> {
    let start = Instant::now();

    let user = handle_signup_form(form);

    let count = distinct_check( "userid".to_string(), user.userid).await;

    if count > 0 {
        let b = render!(SIGNUPFAILURE);
        return Html(b);
    }else {
        let user = insert_into_users(user).await;
    
        println!("successfully inserted- userid: {}, name: {}", user.userid, user.name);
    }

    let u = render!(SIGNUPSUCCESS);

    let duration = start.elapsed();
    println!("SIGNUP activity-time elapsed: {:?}", duration);

    Html(u)
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------

//login
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

fn handle_login_form(form: axum::Form<LoginInput>) -> i32 {
    form.id
}

pub async fn loginactivity(form: axum::Form<LoginInput>) -> Html<String>{
    let userid = handle_login_form(form);
    let count = distinct_check("userid".to_string(), userid).await;

    if count > 0 {
        let r = render!(LOGINACTIVITY, userid => userid);
        return Html(r);
    }

    let b = render!(LOGINFAILURE, userid => userid);

    Html(b)
}

//--------------------------------------------------------------------------------------------------------------------

//userpage
pub async fn userpage(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let user = select_from_users(userid).await;

    let r = render!(USERPAGE, user => user);

    let duration = start.elapsed();
    println!("rendering USERPAGE-time elapsed: {:?}", duration);

    Html(r)
}

//-----------------------------------------------------------------------------------------------------------------------

//history

#[derive(Debug,Serialize)]
pub struct Transaction {
    pub date: String,
    pub userid: i32,
    pub amount: f64,
    pub category: String
}

pub async fn history(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let transactions = select_from_transactions(userid).await;

    let r = render!(HISTORY, transactions => transactions, userid => userid);

    let duration = start.elapsed();
    println!("rendering HISTORY-time elapsed: {:?}", duration);

    Html(r)
}

//deposit
pub async fn deposit(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();
    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let user = select_from_users(userid).await;

    let q = render!(DEPOSIT, user => user);

    let duration = start.elapsed();
    println!("rendering DEPOSIT-time elapsed: {:?}", duration);

    Html(q)
}

#[derive(Deserialize)]
pub struct UserInput {
    userid: i32, 
    amount: f64,
    category: String
}

fn handle_transaction_form(form: axum::Form<UserInput>) -> Transaction{
    let current_date = Utc::now().naive_utc();
    let date = current_date.to_string();

    let transaction = Transaction{
        date,
        userid: form.userid,
        amount: form.amount,
        category: form.category.to_string()
    };

    transaction
}

pub async fn depositactivity(form: axum::Form<UserInput>) -> Html<String> {
    let start = Instant::now();

    let transaction = handle_transaction_form(form);

    insert_into_transactions(transaction.date, transaction.userid, transaction.amount, transaction.category.to_string()).await;

    update(transaction.userid, transaction.amount).await;

    let user = select_from_users(transaction.userid).await;
    
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

    let user = select_from_users(userid).await;

    let s = render!(WITHDRAW, user => user);

    let duration = start.elapsed();
    println!("rendering WITHDRAW-time elapsed: {:?}", duration);

    Html(s)
}

pub async fn withdrawactivity(form: axum::Form<UserInput>) -> Html<String> {
    let start = Instant::now();

    let transaction = handle_transaction_form(form);

    let user = select_from_users(transaction.userid).await;

    if user.balance < transaction.amount {
        let u = render!(FAILURE, user => user);

        let duration = start.elapsed();
        println!("WITHDRAW activity(failure)-time elapsed: {:?}", duration);

        return Html(u);
    } else {
        update(transaction.userid, -transaction.amount).await;
        insert_into_transactions(transaction.date, transaction.userid, -transaction.amount, transaction.category.to_string()).await;
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