pub const HOME: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>passbook</title>

  <style>
    .center{
      position:relative;
      width:100%;
      height:2em;
    }
    .center a{
      position:absolute;
      top:50%;
      left:50%;
      transform: translate(-50%,-50%);
    }
    
  </style>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="cornsilk" alink="ffe990" vlink="cornsilk">
<br><br><br><br><br><br><br><br><br><br><br><br><br><br>
    <p style="text-align:center; font-size:70px;"><b>HOMEPAGE</b></p>
    <div class="center">
    <a href="/login" style="text-decoration:none; font-size: 30px;">Login</a><br>
    </div>
    <br>
    <div class="center">
    <a href="/signup" style="text-decoration:none; font-size: 30px;">Sign Up</a><br><br><br>
    </div>
</body>
</html>
"#;

pub const SIGNUP: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>new account</title>

  <style>
    .form{
      position:relative;
      width:100%;
      height:1em;
    }
    .form form{
      position:absolute;
      top:50%;
      left:50%;
      transform: translate(-50%,-50%);
    }
    
  </style>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk" link="ivory" alink="cornsilk" vlink="ivory">
<br><br><br><br><br><br><br><br><br><br><br><br><br><br>
    <p style="text-align:center; font-size:50px"><b>SIGNUP</b></p>
    <br><br>
    <div class="form">
    <form action = "/signup" method = "post">
        <label for="userid">UserID:</label><br>
        <input type = "number" name = "userid" min = "1" style="border:none; border-bottom:2px solid cornsilk; background-color:#121212; color:cornsilk; outline:none"><br>
        <label for="name">Name:</label><br>
        <input type = "text" name = "name" style="border:none; border-bottom:2px solid cornsilk; background-color:#121212; color:cornsilk; outline:none"><br>
        <input type = "submit" value = "submit" style="background-color:#121212; border:none; color:#121212;">
    </form>
    </div>
</body>
</html>
"#;

pub const SIGNUPFAILURE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>failed</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p>UserID already exists</p>
    <p><a href="/signup" style="text-decoration:none;">Try Again</a></p>

</body>
</html>
"#;

pub const SIGNUPSUCCESS: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>success</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p>Account added successfully</p>
    <p><a href="/login" style="text-decoration:none;">Login</a></p>
</body>
</html>
"#;

pub const LOGIN: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>login</title>

  <style>
    .form{
      position:relative;
      width:100%;
      height:1em;
    }
    .form form{
      position:absolute;
      top:50%;
      left:50%;
      transform: translate(-50%,-50%);
    }
    
  </style>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <br><br><br><br><br><br><br><br><br><br><br><br><br><br>
    <p style="text-align:center; font-size:50px"><b>LOGIN</b></p>
    <br>
    <div class="form">
    <form action = "/login" method = "post">
        <label for="userid">account id:</label><br>
        <input type = "number" name = "userid" min = "1" style="border:none; border-bottom:2px solid cornsilk; background-color:#121212; color:cornsilk; outline:none"><br>
        <input type = "submit" value = "submit" style="background-color:#121212; border:none; color:#121212;">
    </form>
    </div>
</body>
</html>
"#;

pub const LOGINACTIVITY: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="refresh" content="0; URL=/userpage/{{ userid }}" />

  <title>loading</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:#94B3FD;">
</body>
</html>
"#;

pub const LOGINFAILURE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>failed</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p>Account does not exist</p>
    <p><a href="/login" style="text-decoration:none;">Login</a></p>
    <p><a href="/signup" style="text-decoration:none;">Sign up</a></p>
</body>
</html>
"#;

pub const USERPAGE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>{{ user.name }}'s dashboard</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk; margin:60px;" link="ivory" alink="cornsilk" vlink="ivory">
    <p>Welcome {{ user.name }}</p>

    userid: {{ user.userid }}<br>
    available balance: {{ user.balance }}<br><br><br>

    <p style="font-size:25px;">Actions:<p>
    <a href="/deposit/{{ user.userid }}" style="text-decoration:none;">Deposit</a><br><br>
    <a href="/withdraw/{{ user.userid }}" style="text-decoration:none;">Withdraw</a><br><br>
    <a href="/history/{{ user.userid }}" style="text-decoration:none;">Full transaction history</a><br><br>
    <a href="/delete/{{ user.userid }}" style="text-decoration:none;">Delete account</a>
</body>
</html>
"#;

pub const HISTORY: &str = r#"
<!doctype html>
<html>
        <head>
            <title>user history</title>
        </head>
          <body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
            <h1>transactions</h1>
            
                <p>
                    {% for transaction in transactions %}
                        {{ transaction.amount }}<br>
                        {{ transaction.date }}<br>
                        {{ transaction.category }}<br>
                        <br>
                        -----------------------------------------------------------------------
                        <br><br>
                    {% endfor %}
                </p>
            
            <p><a href="/userpage/{{ userid }}" style="text-decoration:none;">back to main menu</a></p>
        </html>
"#;

pub const DEPOSIT: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>deposit page</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p style="font-size:40px;">Deposit</p>
    <form action = "/deposit/{{ user.userid }}" method = "post">
        <input type="hidden" id = "userid" name="userid" value="{{ user.userid }}">
        <label for="amount">amount to be deposited:</label><br>
        <input type = "number" step = "0.01" name = "amount" min = "0" style="border:none; border-bottom:1px solid cornsilk; background-color:#121212; color:cornsilk; outline:none"><br>
        <select name="category" id="category" style="background-color:#121212; border:none; color:cornsilk; outline:none;">
          <option value="salary">salary</option>
          <option value="misc">misc</option>
        </select>
        <input type = "submit" value = "submit" style="background-color:#121212; border:none; color:#121212;">
    </form>
</body>
</html>
"#;

pub const WITHDRAW: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>withdraw page</title>
</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p style="font-size:40px;">Withdraw</p>
    <form action = "/withdraw/{{ user.userid }}" method = "post">
        <input type="hidden" id = "userid" name="userid" value="{{ user.userid }}">
        <label for="amount">Amount to be withdrawn:</label><br>
        <input type = "number" step = "0.01" name = "amount" min = "0" style="border:none; border-bottom:1px solid cornsilk; background-color:#121212; color:cornsilk; outline:none"><br><br>
        <label for="category">category:</label><br>
        <select name="category" id="category" style="background-color:#121212;border:none;color:cornsilk;outline:none;">
          <option value="bills">bills</option>
          <option value="food">food</option>
          <option value="vacation">vacation</option>
          <option value="savings">savings</option>
          <option value="misc">misc</option>
        </select>
        <input type = "submit" value = "submit" style="background-color:#121212; border:none; color:#121212;">
    </form>
</body>
</html>
"#;

pub const SUCCESS: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>success</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p>Transaction was successful</p>
    <p><a href="/userpage/{{ user.userid }}" style="text-decoration:none;">back to main menu</a></p>

</body>
</html>
"#;

pub const FAILURE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>failed</title>

</head>

<body style="background-color:#121212; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p>Not enough balance</p>
    <p><a href="/userpage/{{ user.userid }}" style="text-decoration:none;">Back to dashboard</a></p>

</body>
</html>
"#;

pub const REMOVESUCCESS: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>success</title>

</head>

<body style="background-color:black; font-family:helvetica; color:cornsilk;" link="ivory" alink="cornsilk" vlink="ivory">
    <p>Account removed successfully</p>
    <p><a href="/" style="text-decoration:none;">Back to main menu</a></p>
</body>
</html>
"#;