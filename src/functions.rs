use axum::response::Html;
use minijinja::render;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use chrono::Utc;
use std::fs::File;
use std::io::Read;

use crate::database::{insert_into_users, select_from_users, update, delete_row, select_from_transactions, insert_into_transactions, distinct_check};

#[derive(Serialize)]
pub struct User{
    pub userid:i32,
    pub name:String,
    pub balance:f64
}

#[derive(Debug,Serialize)]
pub struct Transaction {
    pub date: String,
    pub userid: i32,
    pub amount: f64,
    pub category: String
}

#[derive(Deserialize, Debug)]
pub struct Data{
    pub userid: Option<String>,
    pub name: Option<String>,
    pub balance: Option<String>,
    pub amount: Option<String>,
    pub category: Option<String>
}

async fn get_html_file(filename: &str) -> Result<String, std::io::Error> {
    let path = format!("src/templates/{}.html", filename);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub async fn home() -> Html<String>  {
    let start = Instant::now();
    
    let render = get_html_file("homepage").await.unwrap();

    let duration = start.elapsed();
    println!("rendering HOME-time elapsed: {:?}\n", duration);

    Html(render)
}

//signup
pub async fn signup() -> Html<String> {
    let start = Instant::now();

    let render = get_html_file("signup").await.unwrap();

    let duration = start.elapsed();
    println!("rendering SIGNUP-time elapsed: {:?}\n", duration);

    Html(render)
}

fn handle_signup_request(data: axum::extract::Json<Data>) -> User{
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

pub async fn signupactivity(data: axum::extract::Json<Data>) -> Html<String> {
    let start = Instant::now();

    let user = handle_signup_request(data);

    let count = distinct_check( "userid".to_string(), user.userid).await;

    if count > 0 {
        let render = get_html_file("signupfailure").await.unwrap();

        println!("could not sign up user(userid is already taken)\n");

        let duration = start.elapsed();

        println!("SIGNUP activity(failure)-time elapsed: {:?}", duration);
        
        return Html(render);
    }else {
        let user = insert_into_users(user).await;
    
        println!("successfully inserted- userid: {}, name: {}\n", user.userid, user.name);
    }

    let render = get_html_file("signupsuccess").await.unwrap();

    let duration = start.elapsed();
    println!("SIGNUP activity(success)-time elapsed: {:?}", duration);

    Html(render)
}

//login
pub async fn login() -> Html<String> {
    let start = Instant::now();

    let render = get_html_file("login").await.unwrap();

    let duration = start.elapsed();
    println!("rendering LOGIN-time elapsed: {:?}\n", duration);

    Html(render)
}

fn handle_login_form(data: axum::extract::Json<Data>) -> i32 {
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

pub async fn loginactivity(data: axum::extract::Json<Data>) -> Html<String>{
    let userid = handle_login_form(data);

    let count = distinct_check("userid".to_string(), userid).await;

    if count > 0 {
        let html = get_html_file("loginactivity").await.unwrap();
        let render = render!(&html, userid => userid);
        println!("logged user in successfully(userid = {})\n", userid);
        return Html(render);
    }

    let html = get_html_file("loginfailure").await.unwrap();
    let render = render!(&html, userid => userid);
    println!("login failed(userid does not exist)\n");
    Html(render)
}

//userpage
pub async fn userpage(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let user = select_from_users(userid).await;

    let html = get_html_file("userpage").await.unwrap();
    let r = render!(&html, user => user);

    let duration = start.elapsed();
    println!("rendering USERPAGE-time elapsed: {:?}\n", duration);

    Html(r)
}

//history
pub async fn history(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let transactions = select_from_transactions(userid).await;

    let html = get_html_file("history").await.unwrap();
    let r = render!(&html, transactions => transactions, userid => userid);

    let duration = start.elapsed();
    println!("rendering HISTORY-time elapsed: {:?}\n", duration);

    Html(r)
}

//deposit
pub async fn deposit(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();
    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let user = select_from_users(userid).await;

    let html = get_html_file("deposit").await.unwrap();
    let q = render!(&html, user => user);

    let duration = start.elapsed();
    println!("rendering DEPOSIT-time elapsed: {:?}\n", duration);

    Html(q)
}

fn handle_transaction_form(data: axum::extract::Json<Data>) -> Transaction{
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

pub async fn depositactivity(data: axum::extract::Json<Data>) -> Html<String> {
    let start = Instant::now();

    let transaction = handle_transaction_form(data);

    insert_into_transactions(transaction.date, transaction.userid, transaction.amount, transaction.category.to_string()).await;

    update(transaction.userid, transaction.amount).await;

    let user = select_from_users(transaction.userid).await;
    
    let html = get_html_file("transactionsuccess").await.unwrap();
    let y = render!(&html, user => user);

    let duration = start.elapsed();
    println!("amount deposited: {} - time elapsed: {:?}", transaction.amount, duration);

    Html(y)
}

//withdraw
pub async fn withdraw(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    let user = select_from_users(userid).await;

    let html = get_html_file("withdraw").await.unwrap();
    let s = render!(&html, user => user);

    let duration = start.elapsed();
    println!("rendering WITHDRAW-time elapsed: {:?}\n", duration);

    Html(s)
}

pub async fn withdrawactivity(data: axum::extract::Json<Data>) -> Html<String> {
    let start = Instant::now();

    let transaction = handle_transaction_form(data);

    let user = select_from_users(transaction.userid).await;

    if user.balance < transaction.amount {
        let html = get_html_file("transactionfailure").await.unwrap();
        let u = render!(&html, user => user);

        let duration = start.elapsed();
        println!("withdraw failed(not enough balance)-time elapsed: {:?}\n", duration);

        return Html(u);
    } else {
        update(transaction.userid, -transaction.amount).await;
        insert_into_transactions(transaction.date, transaction.userid, -transaction.amount, transaction.category.to_string()).await;
    }
    
    let html = get_html_file("transactionsuccess").await.unwrap();
    let x = render!(&html, user => user);

    let duration = start.elapsed();
    println!("amount withdrawn: {}-time elapsed: {:?}\n", transaction.amount, duration);

    Html(x)
}


//delete user
pub async fn delete(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let userid:i32 = active_user.parse().unwrap();

    delete_row(userid).await;

    let html = get_html_file("removeaccountsuccess").await.unwrap();

    let duration = start.elapsed();
    println!("deleted a user's file-time elapsed: {:?}\n", duration);

    Html(html)
}


