use axum::response::Html;
use minijinja::render;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use chrono::Utc;
use axum::Json;
use axum::http::StatusCode;
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

#[derive(Deserialize)]
pub struct FormInput{
    pub userid: Option<String>,
    pub name: Option<String>,
    pub balance: Option<f64>,
    pub amount: Option<f64>,
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

fn handle_signup_form(form: axum::Form<FormInput>) -> User{
    let start = Instant::now();

    let input = FormInput {
        userid: form.userid.clone(),
        name: form.name.clone(),
        balance: None,
        amount: None,   
        category: None
    };

    let userid = input.userid.expect("userid field empty").parse::<i32>().unwrap();
    let name = input.name.expect("name field empty");

    let user = User{
        userid,
        name,
        balance: 0.0
    };

    println!("Form input- userid:{}, name: {}", user.userid, user.name);

    let duration = start.elapsed();
    println!("HTML form put into a User struct- time elapsed: {:?}\n", duration);

    user
}

pub async fn signupactivity(form: axum::Form<FormInput>) -> Html<String> {
    let start = Instant::now();

    let user = handle_signup_form(form);

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

fn handle_login_form(form: axum::Form<FormInput>) -> i32 {
    let input = FormInput {
        userid: form.userid.clone(),
        name: None,
        balance: None,
        amount: None,
        category: None
    };

    let userid = input.userid.expect("userid field empty").parse::<i32>().unwrap();

    println!("logging in with userid: {}\n", userid);
    userid
}

pub async fn loginactivity(form: axum::Form<FormInput>) -> Html<String>{
    let userid = handle_login_form(form);
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

fn handle_transaction_form(form: axum::Form<FormInput>) -> Transaction{
    let input = FormInput {
        userid: form.userid.clone(),
        name: None,
        balance: None,
        amount: form.amount,
        category: form.category.clone()
    };

    let current_date = Utc::now().naive_utc();
    let date = current_date.to_string();

    let userid = input.userid.expect("userid field empty").parse::<i32>().unwrap();
    let amount = input.amount.expect("amount field empty");
    let category = input.category.expect("category field empty");

    let transaction = Transaction{
        date,
        userid,
        amount,
        category
    };

    println!("HTML form is stored in Transaction struct");
    println!("transaction details- userid: {}, amount: {}, category: {}\n", transaction.userid, transaction.amount, transaction.category);

    transaction
}

pub async fn depositactivity(form: axum::Form<FormInput>) -> Html<String> {
    let start = Instant::now();

    let transaction = handle_transaction_form(form);

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

pub async fn withdrawactivity(form: axum::Form<FormInput>) -> Html<String> {
    let start = Instant::now();

    let transaction = handle_transaction_form(form);

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    userid: Option<i32>,
    name: Option<String>,
}

pub async fn handle_signup_post(data: axum::extract::Json<Data>) -> Result<Json<Data>, String>{
    let request_data = Data {
        userid: data.userid,
        name: data.name.clone()
    };

    let userid = request_data.userid.expect("userid field empty");
    let name = request_data.name.clone().expect("name field empty");

    println!("received data: name = {}, userid = {}\n", name, userid);

    let user = User{
        userid,
        name, 
        balance: 0.0 
    };

    let count = distinct_check( "userid".to_string(), userid).await;

    if count > 0 {
        let status_code = format!("status code: {}", StatusCode::BAD_REQUEST);
        eprintln!("user already exists\n");
        return Err(status_code)
    }else {
        let user = insert_into_users(user).await;
        
        println!("successfully inserted- userid: {}, name: {}\n", user.userid, user.name);
    }

    Ok(Json(request_data))
}
//curl -X POST -H "Content-Type: application/json" -d '{"name": "uuqx", "userid":56}' http://localhost:3000/signuppost


