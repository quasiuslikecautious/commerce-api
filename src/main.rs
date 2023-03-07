// TODO:
// [ ] nonce for auth (?)
//     [x] fix custom session store to save session if new
// [ ] connect JWT and cooke session
// [x] pagination
//      [x] add to /item/all route
// [ ] email service
//      [ ] password reset
//      [ ] email verification
// [ ] encrypt data at rest
// [ ] truncate table from diesel
// [ ] crate documentation
// [ ] openapi documentation

//! Commerce API written with Axum and Diesel
//! 
//! This is the binary for the commerce RESTful API written with Axum RUST. 
//! This API uses diesel to interact with a PostgreSQL database and supports 
//! https, JWT, access control, encrypted stored passwords, and logging. 

mod db;
mod jwt;
mod middlewares;
mod net;
mod sessionstore;

use axum::{
    extract::{ Json, Path, Query },
    http::{ header::SET_COOKIE, StatusCode },
    response::AppendHeaders,
    routing::{ get, post, put, },
    Router,
};
use axum_auth::AuthBearer;
use axum_server::tls_rustls::RustlsConfig;
use axum_sessions::extractors::{ ReadableSession, WritableSession };
use diesel::{ prelude::*, RunQueryDsl, QueryDsl };
use dotenvy::dotenv;
use log::{ debug, trace, info };
use std::{ env, net::SocketAddr, path::PathBuf, time::Duration, collections::HashMap };
use uuid::{ uuid, Uuid };
use validator::Validate;

use crate::db::*;
use crate::jwt::*;
use crate::middlewares::*;
use crate::net::*;

type ApiResponse<T> = Result<Json<T>, ErrorResponse>;
type ApiResponseWithHeaders<T> = Result<(AppendHeaders<Vec<(String, String)>>, Json<T>), ErrorResponse>;

fn parse_path_uuid(params: HashMap<String, String>, key: &str) -> Result<Uuid, ErrorResponse> {
    let param_value = params.get(key)
        .ok_or(AppError::as_response(StatusCode::NOT_FOUND, "Not Found"))?;

    Uuid::parse_str(param_value)
        .or(Err(AppError::as_response(StatusCode::NOT_FOUND, "Not Found")))
}

/// Inits the API and starts the socket
/// 
/// The main function of the binary. Configures the log, ports, SSL 
/// certificates, and configures the axum app with a trace layer and cors 
/// layer, along with all routes
/// 
/// # Panics
/// This function will panic if the logger fails to init, SSL cert files fail
/// to load, or the server fails to bind to it's specified socket, or fails to
/// start for any reason.
/// 
/// # Examples
/// 
/// ```sh
/// # To start the server, use
/// cargo run
/// 
/// # Otherwise if you have cargo watch and want to use that instead, make sure
/// to exclude the log folder to avoid continuous restarts
/// cargo watch -x run -i log 
/// ``` 
/// 
#[tokio::main]
async fn main() {
    // init logger
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    trace!("main");

    // setup https
    let ports = Ports {
        http: 7878,
        https: 8000,
    };
    
    // build our application
    info!("Booting up server...");
    let user_routes = Router::new()
        .route("/:id", get(get_user));

    let auth_routes = Router::new()
        .route("/nonce", get(nonce))
        .route("/signin", post(signin))
        .route("/signout", get(signout))
        .route("/signup", put(signup));

    let debug_routes = Router::new()
        .route("/helloworld", get(|| async { "Hello, World!" }))
        .route("/dummyauth", get(dummy_auth))
        .route("/admin", get(admin))
        .route("/sleep/:time", get(get_sleep));

    let session_routes = Router::new()
        .route("/write", get(write_session))
        .route("/read", get(read_session))
        .route("/destroy", get(destroy_session));

    let item_routes = Router::new()
        .route("/:id", get(get_item))
        .route("/all", get(get_items))
        .route("/", post(create_item));

    let all_routes = Router::new()
        .nest("/user", user_routes)
        .nest("/auth", auth_routes)
        .nest("/debug", debug_routes)
        .nest("/session", session_routes)
        .nest("/item", item_routes);

    // get api version from .env
    dotenv().ok();
    let api_version = env::var("API_VERSION").expect("API_VERSION must be set");

    let api_routes = Router::new()
        .nest(format!("/api/v{}", api_version).as_str(), all_routes);
    
    let app = with_middleware_stack(api_routes)
        .fallback(fallback);

    debug!("Spawning http to https redirect server");
    tokio::spawn(redirect_http_to_https(ports));

    debug!("Grabbing Self Signed Certs for https");
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("localhost.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("localhost.key"),
    )
    .await
    .unwrap();

    // run it with hyper on localhost:8000
    let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    info!("listening at {}", addr.to_string());
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn fallback() -> ErrorResponse {
    AppError::as_response(StatusCode::NOT_FOUND, "Not Found")
}

async fn dummy_auth(
    mut session: WritableSession
) -> String {
    let dummy_id = Uuid::new_v4();
    
    session
        .insert("user_id", &dummy_id)
        .expect("Could not dummy auth");

    dummy_id.to_string()
}

async fn get_sleep(
    Path(time): Path<u64>
) -> String {
    async_std::task::sleep(Duration::from_millis(time)).await;
    format!("slept for {} ms", time)
}

async fn destroy_session(
    mut session: WritableSession
) {
    session.destroy();
}

async fn read_session(
    session: ReadableSession
) {
    let Some(id) = session.get::<Uuid>("sid")
    else {
        return;
    };

    println!("{}", id);
}

async fn write_session(
    mut session: WritableSession
) {
    session
        .insert("sid", Uuid::new_v4())
        .expect("Could not store the answer.");
}

/// Test route for access control checks. Takes in the user's JWT Bearer Token and verifies they
/// have the 'admin' role uuid.
async fn admin(
    _session: ReadableSession, 
    AuthBearer(token): AuthBearer
) -> ApiResponse<User> {
    debug!("GET request received on /admin route");
    let claims: Claims = match decrypt_jwt(&get_secret(), &token) {
        Ok(data) => data,
        Err(_) => return Err(AppError::as_response(StatusCode::UNAUTHORIZED, "Unauthorized")),
    };

    // TODO
    // add non static checking of role.
    if claims.role == uuid!("9abe48f7-307a-4ee8-929c-843c16cfc75b") {
        debug!("Authorized user, admin request fulfilled, sending JSON response");
        return Ok(Json(User {
            uuid: Some(uuid!("9abe48f7-307a-4ee8-929c-843c16cfc75b")), 
            role: uuid!("9abe48f7-307a-4ee8-929c-843c16cfc75b"),
            email: "a".to_string(),
            password: "b".to_string(),
        }));
    }

    return Err(AppError::as_response(StatusCode::UNAUTHORIZED, "Unauthorized"));
}

/// Due to the way axum session layer currently works, on session load the layer is given a sort of
/// initial session id, and then on session store a new session id is generated and used for the
/// rest of the session. As such, when we receive a request on a route that requires a valid
/// session id to already exist in the database (i.e. the fk requirement of nonces on the sessions
/// table), we must insert the new session id if it does not already exist. Hopefully this is a
/// temporary fix.
async fn redundant_session_guarantee(sid: &str) {
    use crate::schema::sessions::dsl::*;

    let connection = &mut establish_connection();
    connection.build_transaction()
        .read_write()
        .run(|conn| {
              diesel::insert_into(sessions)
                    .values(
                        UserSession::new(
                            sid.to_string(),
                            None,
                            None,
                            None,
                            None,
                            None
                        )
                    )
                    .on_conflict_do_nothing()
                    .execute(conn)
        }).unwrap();
}

///
async fn nonce(session: ReadableSession) -> Result<NonceResponse, ErrorResponse> {
    debug!("GET request received on /nonce route");
    use crate::schema::nonces::dsl::*;

    let sid = session.id();
    let session_nonce = Nonce::new(&sid);

    println!("session nonce: {:?}", &session_nonce);
    redundant_session_guarantee(&sid).await;

    let connection = &mut establish_connection();
    let response = connection.build_transaction()
        .read_write()
        .run(|conn| {
            diesel::insert_into(nonces)
                .values(
                    &session_nonce,
                )
                .on_conflict(session_id)
                .do_update()
                .set((
                    nonce.eq(&session_nonce.nonce),
                    created_at.eq(&session_nonce.created_at),
                    expires_at.eq(&session_nonce.expires_at),
                ))
                .execute(conn)
        });

    println!("after insert");

    trace!("Nonce added for session to database");
    match response {
        Ok(_) => {
            Ok(NoncePayload::as_response(session_nonce.nonce))
        },
        Err(e) => {
            println!("{:?}", e);
            Err(AppError::as_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate nonce"))
        },
    }

}

/// POST route for user authentication.
/// 
/// Route for user authentication. Takes in an email and password, checks database for a match,
/// and the returns the user's uuid, email, role uuid, and a generated JWT to be used for
/// access control and stateless management. Additionally sends a set-cookie header for browser
/// clients
/// 
/// # Errors
/// This route will error if the specified combination of username and password does not exist in
/// the database.
/// 
/// # Examples
/// 
/// ```
/// use axum::{ Router, routing::post };
/// 
/// fn main() {
///     ...
///     let app = Router::new()
///         .route("/auth", post(auth))
///     ...
/// }
/// ```
/// 
/// To access this route, use the following curl:
/// 
/// ```sh
/// curl --insecure -X POST https://localhost:8000/auth \
///     -H 'Content-Type: application/json' \
///     -d '{"email": "<some email>", "password": "<some password>"}'
/// ```
/// 
async fn signin(
    mut session: WritableSession, 
    Json(payload): Json<UserAuth>
) -> ApiResponseWithHeaders<UserAuthPayload> {
    debug!("POST request received on /signin route");
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            users
                .filter(email.eq(&payload.email))
                .filter(password.eq(crypt(&payload.password, password)))
                .first::<User>(conn)
            });

    trace!("Values received from database");

    return match response {
        Ok(user) => {
            debug!("Auth request successfully fulfilled, sending JSON response");
            session.insert("user_id", &user.uuid).expect("Failed to set auth session");
            let payload = UserAuthPayload::from(user);

            Ok( (
                AppendHeaders(
                    vec!((SET_COOKIE.to_string(), get_auth_cookie(&payload.token)))
                )
                , Json(payload) ) )
        },
        Err(_) => Err(AppError::as_response(StatusCode::UNAUTHORIZED, "Failed to authenticate")),
    };
}

async fn signout(
    mut session: WritableSession
) -> ApiResponse<(String,)> {
    debug!("GET request received on /signout route");

    session.destroy();

    Ok(Json(("User successfully logged out".to_string(),)))
}

/// 
async fn signup(
    mut session: WritableSession, 
    Json(payload): Json<UserAuth>
) -> ApiResponse<UserData> {
    debug!("PUT request recieved on /signup route");
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    match payload.validate() {
        Ok(_) => (),
        Err(_) => return Err(AppError::as_response(StatusCode::BAD_REQUEST, "Input validation failed")),
    };

    let response = connection.build_transaction()
        .read_write()
        .run(|conn| {
            diesel::insert_into(users)
                .values((
                    email.eq(payload.email),
                    password.eq(crypt(payload.password, gen_salt("bf"))),
                ))
                .get_result::<User>(conn)
        });
    
    trace!("Values received from database");

    match response {
        Ok(body) => {
            debug!("User request successfully fulfilled, user created, sending JSON response");
            session.insert("user_id", body.uuid).expect("Failed to set user auth session");
            Ok(Json(UserData::from(body)))
        },
        Err(_) => Err(AppError::as_response(StatusCode::INTERNAL_SERVER_ERROR, "Unable to create user"))
    }
}

/// GET route for getting information related to the specified user associated with the uuid in the
/// route's path. First checks the provided JWT to verify it is valid, and that it is assigned to
/// user that is attempting to access the route., or that the user specified in the JWT has 'admin'
/// role.
async fn get_user(
    session: ReadableSession, 
    Path(params): Path<HashMap<String, String>>
) -> ApiResponse<UserData> {
    debug!("GET request received on /user/:uuid route");
    use crate::schema::users::dsl::*;

    let path_user_id = parse_path_uuid(params, "id")?;

    let session_user_id = session.get::<Uuid>("user_id");

    trace!("Fallback role check for 'admin'");
    if session_user_id.is_none() || session_user_id.unwrap() != path_user_id {
        return Err(AppError::as_response(StatusCode::UNAUTHORIZED, "Unauthorized"));
    }

    let connection = &mut establish_connection();

    let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            users
                .filter(uuid.eq(&path_user_id))
                .first::<User>(conn)
        });

    trace!("Values received from database");

    match response {
        Ok(body) => {
            debug!("User request successfully fulfilled, sending JSON response");
            Ok(Json(UserData::from(body)))
        },
        Err(_) => Err(AppError::as_response(StatusCode::NOT_FOUND, "User not found")),
    }
}

async fn get_item(
    _session: ReadableSession, 
    Path(params): Path<HashMap<String, String>>
) -> ApiResponse<Deal> {
    debug!("GET request received on /item/:uuid route");
    use crate::schema::deals::dsl::*;

    let connection = &mut establish_connection();

    let path_item_id = parse_path_uuid(params, "id")?;

    let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            deals
                .filter(uuid.eq(path_item_id))
                .first::<Deal>(conn)
        });

    trace!("Values received from database");

    match response {
        Ok(body) => {
            debug!("Item request successfully fulfilled, sending JSON response");
            Ok(Json(body))
        },
        Err(_) => Err(AppError::as_response(StatusCode::NOT_FOUND, "Item not found")),
    }
}

async fn create_item(
    _session: ReadableSession, 
    Json(payload): Json<Deal>
) -> ApiResponse<Deal> {
    debug!("PUT request received on /item route");
    use crate::schema::deals::dsl::*;

    let connection = &mut establish_connection();

    let response = connection.build_transaction()
        .read_write()
        .run(|conn| {
            diesel::insert_into(deals)
                .values((
                    name.eq(payload.name),
                    image.eq(payload.image),
                    price.eq(payload.price),
                ))
                .get_result::<Deal>(conn)
            });
    
    trace!("Values received from database");
    
    match response {
        Ok(body) => {
            debug!("Item request successfully fulfilled, item created, sending JSON response");
            Ok(Json(body))
        },
        Err(_) => Err(AppError::as_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create item"))
    }
}

async fn get_items(
    _session: ReadableSession,
    pagination: Option<Query<Pagination>>
) -> ApiResponse<Items> {
    debug!("GET request received on /items route");
    use crate::schema::deals::dsl::*;

    let Query(pagination) = pagination.unwrap_or_default();

    let connection = &mut establish_connection();

    let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            deals
                .limit(pagination.get_limit())
                .offset(pagination.get_offset())
                .load::<Deal>(conn)
        });

    trace!("Values received from database");
    
    match response {
        Ok(body) => {
            debug!("Items request successfully fulfilled, sending JSON array response");
            Ok(Json(Items { items: body, }))
        },
        Err(_) => Err(AppError::as_response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get items")),
    }
}
