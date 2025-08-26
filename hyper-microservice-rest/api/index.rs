use std::sync::{Arc, Mutex};
use slab::Slab;
use std::fmt;
use vercel_runtime::{run, Body as VercelBody, Error, Request as VercelRequest, Response as VercelResponse, StatusCode};
use http::Method;

const INDEX: &str = r#"
<!doctype html>
<html>
    <head>
        <title>Rust REST Microservice - Hyper 1.7.0 on Vercel</title>
    </head>
    <body>
        <h3>Rust REST Microservice - Hyper 1.7.0 on Vercel</h3>
        <p>This microservice demonstrates:</p>
        <ul>
            <li><strong>hyper 1.7.0</strong> - HTTP library</li>
            <li><strong>Vercel Runtime</strong> - Serverless deployment</li>
            <li><strong>REST API</strong> - Full CRUD operations</li>
            <li><strong>User management</strong> - Create, read, update, delete users</li>
        </ul>
        <h4>API Endpoints:</h4>
        <ul>
            <li><strong>GET /</strong> - This page</li>
            <li><strong>GET /user/{id}</strong> - Get user by ID</li>
            <li><strong>POST /user/</strong> - Create new user</li>
            <li><strong>PUT /user/{id}</strong> - Update user</li>
            <li><strong>DELETE /user/{id}</strong> - Delete user</li>
        </ul>
        <p>Deployed on Vercel as a serverless function!</p>
    </body>
</html>
"#;

type UserId = u64;

struct UserData;

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{}")
    }
}

type UserDb = Arc<Mutex<Slab<UserData>>>;

const USER_PATH: &str = "/user/";

async fn handler(req: VercelRequest) -> Result<VercelResponse<VercelBody>, Error> {
    let path = req.uri().path();
    let user_db = Arc::new(Mutex::new(Slab::<UserData>::new())); // In production, this would be shared state
    
    let response = match (req.method(), path) {
        (&Method::GET, "/") => {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(VercelBody::from(INDEX))
                .unwrap()
        },
        (method, path) if path.starts_with(USER_PATH) => {
            let user_id = path.trim_start_matches(USER_PATH)
                .parse::<UserId>()
                .ok()
                .map(|x| x as usize);
            let mut users = user_db.lock().unwrap();
            
            match (method, user_id) {
                (&Method::GET, Some(id)) => {
                    if let Some(data) = users.get(id) {
                        Response::new(VercelBody::from(data.to_string()))
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },
                (&Method::POST, None) => {
                    let id = users.insert(UserData);
                    Response::new(VercelBody::from(id.to_string()))
                },
                (&Method::POST, Some(_)) => {
                    response_with_code(StatusCode::BAD_REQUEST)
                },
                (&Method::PUT, Some(id)) => {
                    if let Some(user) = users.get_mut(id) {
                        *user = UserData;
                        response_with_code(StatusCode::OK)
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },
                (&Method::DELETE, Some(id)) => {
                    if users.contains(id) {
                        users.remove(id);
                        response_with_code(StatusCode::OK)
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },
                _ => {
                    response_with_code(StatusCode::METHOD_NOT_ALLOWED)
                },
            }
        },
        _ => {
            response_with_code(StatusCode::NOT_FOUND)
        },
    };
    
    Ok(response.map(VercelBody::from))
}

fn response_with_code(status_code: StatusCode) -> Response<VercelBody> {
    Response::builder()
        .status(status_code)
        .body(VercelBody::from(""))
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
} 