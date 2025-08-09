use rand::{thread_rng, Rng};
use include_dir::{include_dir, Dir};
use tera::{Tera, Context};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use crate::qd::state::GameState;
use crate::bot::base::Bot;
use crate::bot::collections::map_bot_string;

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/static");
static INDEX_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/index.html.tera"));

struct AppData {
    bot: Box<dyn Bot>,
    token: String,
    tera: Tera,
    use_token: bool,
}

#[derive(Deserialize)]
struct Info {
    token: Option<String>
}

#[derive(Deserialize)]
struct GameStateRepr {
    wqueen: u8,
    bqueen: u8,
    blocks: String,
    is_white_turn: bool,
}

impl GameStateRepr {
    fn to_game_state(&self) -> GameState {
        GameState::new(
            Some(self.wqueen),
            Some(self.bqueen),
            Some(u64::from_str_radix(&self.blocks, 10).unwrap()),
            Some(self.is_white_turn),
        )
    }
}

#[derive(Deserialize)]
struct Data {
    state_repr: GameStateRepr
}

#[derive(Serialize)]
struct Response {
    move_made: u8,
    code: u32
}

fn random_hex_string(len: usize) -> String {
    let mut rng = thread_rng();
    let bytes: Vec<u8> = (0..len).map(|_| rng.r#gen()).collect();
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

async fn bot_endpoint(
    data: web::Data<AppData>,
    req: HttpRequest,
    payload: web::Json<Data>
) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header.trim_start_matches("Bearer ").to_string())
            } else {
                None
            }
        });
    if token.is_none() && data.use_token {
        return HttpResponse::Unauthorized().body("Missing token");
    }
    if Some(data.token.clone()) != token && data.use_token {
        return HttpResponse::Unauthorized().body("Invalid token");
    }
    let res = data.bot.decide(payload.state_repr.to_game_state());
    let response = Response {
        move_made: res,
        code: 200
    };
    HttpResponse::Ok().json(response)
}

async fn index_endpoint(
    data: web::Data<AppData>,
    info: web::Query<Info>
) -> impl Responder {
    let token = info.token.clone();
    if token.is_none() && data.use_token {
        return HttpResponse::Unauthorized().body("Missing token");
    }
    if Some(data.token.clone()) != token && data.use_token {
        return HttpResponse::Unauthorized().body("Invalid token");
    }
    let mut context = Context::new();
    context.insert("token", &data.token);
    let rendered = data.tera.render("index", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn serve_static(req: HttpRequest) -> impl Responder {
    let path = req.path().strip_prefix("/static/").unwrap().trim_start_matches("/");

    match STATIC_DIR.get_file(path) {
        Some(file) => {
            let body = file.contents();

            let content_type = mime_guess::from_path(path).first_or_octet_stream();

            HttpResponse::Ok()
                .content_type(content_type.as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("File not found"),
    }
}

pub async fn play_bot(bot_string: String, port: u16, use_token: bool) -> std::io::Result<()> {
    let token = random_hex_string(16);
    let bot = map_bot_string(&bot_string);
    if bot.is_none() {
        eprintln!("\"{}\" does not exist", bot_string);
        return Ok(());
    }
    let bot = bot.unwrap();
    
    let mut tera = Tera::default();
    tera.add_raw_template("index", INDEX_FILE)
        .expect("Failed to add template");
    let app = {
        let token = token.clone();
        move || {
            App::new()
                .app_data(web::Data::new(AppData {
                    bot: bot.clone(),
                    token: token.clone(),
                    tera: tera.clone(),
                    use_token: use_token.clone()
                }))
                .route("/", web::get().to(index_endpoint))
                .route("/index.html", web::get().to(index_endpoint))
                .route("/static/{filename:.*}", web::get().to(serve_static))
                .route("/bot", web::post().to(bot_endpoint))
        }
    };
    if use_token {
        println!("Access at localhost:{}/?token={}", port, token);
    } else {
        println!("Access at localhost:{}", port);
    }
    HttpServer::new(app)
        .bind(("127.0.0.1", port))?
        .run()
        .await
}