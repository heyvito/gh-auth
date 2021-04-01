pub fn make_page(title: &str, body: &str) -> String {
    format!(r#"
<!DOCTYPE html>
<html>
    <head>
        <title>{}</title>
        <style>
            body {{
                width: 35em;
                margin: 0 auto;
                font-family: Tahoma, Verdana, Arial, sans-serif;
            }}
        </style>
    </head>
    <body>
        <h1>{}</h1>
        <p>{}</p>
    </body>
</html>
"#, title, title, body)
}

pub fn make_login() -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login</title>
    <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/font-awesome/4.7.0/css/font-awesome.min.css">
    <style>
        #login {{
          font-family: "Helvetica Neue",Helvetica,Arial,sans-serif;
          color: #fff;
          background-color: #444;
          border-color: rgba(0, 0, 0, 0.2);
          position: relative;
          padding: 6px 12px;
          padding-left: 44px;
          text-align: left;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          display: block;
          width: 120px;
          margin-bottom: 0;
          font-size: 14px;
          font-weight: 400;
          line-height: 1.42857143;
          touch-action: manipulation;
          cursor: pointer;
          user-select: none;
          background-image: none;
          border: 1px solid;
          border-radius: 4px;
          text-decoration: none;
        }}

        #login i {{
          position: absolute;
          left: 0;
          top: 0;
          bottom: 0;
          width: 32px;
          line-height: 34px;
          font-size: 1.6em;
          text-align: center;
          border-right: 1px solid rgba(0,0,0,0.2);
        }}

        #container {{
           position: absolute;
           top: 0;
           left: 0;
           right: 0;
           bottom: 0;
           display: flex;
           align-items: center;
           justify-content: center;
        }}
    </style>
</head>
<body>
    <div id="container">
        <a href="/__gh_auth/begin" id="login">
            <i class="fa fa-github"></i> Login with GitHub
        </a>
    </div>
</body>
</html>"#)

}
