//! Live-spectate server: runs games headlessly and streams their event log and
//! state over WebSocket; also replays completed games and serves the React UI.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use cna_engine::{
    default_scenario, fog::DepotView, map::Site, Event, GameState, Hex, Map, Side, Unit,
};
use cna_models::{build_agent, run_streaming, Agent, AgentConfig};
use parking_lot::Mutex;
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::services::{ServeDir, ServeFile};

const EVENT_PACE_MS: u64 = 20;

// ---- WS protocol ----

#[derive(Clone, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum WsMessage {
    Snapshot {
        scenario: String,
        horizon: u32,
        map: Map,
        objectives: Vec<Hex>,
        depots: Vec<DepotView>,
        allied: String,
        axis: String,
        turn: u32,
        active: Side,
        winner: Option<Side>,
        game_over: bool,
        units: Vec<Unit>,
    },
    Events {
        events: Vec<cna_engine::Event>,
    },
    State {
        units: Vec<Unit>,
        turn: u32,
        active: Side,
        winner: Option<Side>,
        game_over: bool,
    },
    Done {
        winner: Option<Side>,
        turns: u32,
        illegal_allied: u32,
        illegal_axis: u32,
    },
    Error {
        message: String,
    },
}

// ---- app state ----

struct GameEntry {
    #[allow(dead_code)]
    id: u64,
    allied: String,
    axis: String,
    tx: broadcast::Sender<WsMessage>,
    snapshot: WsMessage,
    latest: Mutex<Option<WsMessage>>,
}

#[derive(Clone)]
struct AppState {
    games: Arc<Mutex<HashMap<u64, Arc<GameEntry>>>>,
    next_id: Arc<AtomicU64>,
    analyst: Arc<dyn Agent>,
}

fn depots_of(map: &Map) -> Vec<DepotView> {
    let mut out = Vec::new();
    for &h in &map.depots {
        if let Some(m) = map.get(h) {
            if let Some(Site::Depot {
                side,
                name,
                capacity,
            }) = &m.site
            {
                out.push(DepotView {
                    hex: h,
                    name: name.clone(),
                    side: *side,
                    capacity: *capacity,
                });
            }
        }
    }
    out
}

fn snapshot(state: &GameState, allied: &str, axis: &str) -> WsMessage {
    WsMessage::Snapshot {
        scenario: state.scenario_name.clone(),
        horizon: state.turn_horizon,
        map: state.map.clone(),
        objectives: state.map.objectives.clone(),
        depots: depots_of(&state.map),
        allied: allied.to_string(),
        axis: axis.to_string(),
        turn: state.turn,
        active: state.active,
        winner: state.winner,
        game_over: state.game_over,
        units: state.units.values().cloned().collect(),
    }
}

fn state_view(state: &GameState) -> WsMessage {
    WsMessage::State {
        units: state.units.values().cloned().collect(),
        turn: state.turn,
        active: state.active,
        winner: state.winner,
        game_over: state.game_over,
    }
}

fn to_text(msg: &WsMessage) -> Message {
    Message::Text(serde_json::to_string(msg).unwrap().into())
}

/// Load live-game agents from `agents.json` (or `AGENTS_CONFIG`); fall back to mocks.
fn load_live_agents() -> Vec<AgentConfig> {
    let path = std::env::var("AGENTS_CONFIG").unwrap_or_else(|_| "agents.json".to_string());
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Ok(configs) = serde_json::from_str::<Vec<AgentConfig>>(&text) {
            if configs.len() >= 2 {
                return configs;
            }
        }
    }
    vec![
        AgentConfig::mock("mock-a"),
        AgentConfig::mock("mock-b"),
    ]
}

fn build_analyst() -> Arc<dyn Agent> {
    let configs = load_live_agents();
    Arc::from(build_agent(&configs[0]))
}

// ---- handlers ----

async fn health() -> Json<Value> {
    Json(json!({ "ok": true }))
}

async fn analyze(Path(file): Path<String>, State(app): State<AppState>) -> Response {
    let path = format!("games/{file}");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "replay not found" })),
        )
            .into_response();
    };
    let Ok(record): Result<Value, _> = serde_json::from_str(&text) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid replay" })),
        )
            .into_response();
    };

    let events: Vec<Event> = record["events"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|e| serde_json::from_value(e.clone()).ok())
                .collect()
        })
        .unwrap_or_default();

    let summary = cna_analyzer::summarize(&events);
    let narrative = match cna_analyzer::narrate(&summary, app.analyst.as_ref()).await {
        Ok(n) => n,
        Err(e) => format!("(narrative unavailable: {e})"),
    };

    Json(json!({
        "allied": record["allied"],
        "axis": record["axis"],
        "summary": summary,
        "narrative": narrative,
    }))
    .into_response()
}

async fn start_game(State(app): State<AppState>) -> Json<Value> {
    let id = app.next_id.fetch_add(1, Ordering::SeqCst);
    let seed = id.wrapping_add(1000);
    let configs = load_live_agents();
    let allied_name = configs[0].name.clone();
    let axis_name = configs[1].name.clone();
    let state = GameState::new(default_scenario(), seed);

    let snap = snapshot(&state, &allied_name, &axis_name);
    let (tx, _rx) = broadcast::channel::<WsMessage>(512);
    let entry = Arc::new(GameEntry {
        id,
        allied: allied_name.clone(),
        axis: axis_name.clone(),
        tx: tx.clone(),
        snapshot: snap,
        latest: Mutex::new(None),
    });
    app.games.lock().insert(id, entry.clone());

    let allied: Arc<dyn Agent> = Arc::from(build_agent(&configs[0]));
    let axis: Arc<dyn Agent> = Arc::from(build_agent(&configs[1]));

    // Paced event fan-out: driver -> mpsc -> (delay) -> broadcast.
    let (ev_tx, mut ev_rx) = tokio::sync::mpsc::unbounded_channel::<WsMessage>();
    let btx = tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = ev_rx.recv().await {
            let _ = btx.send(msg);
            tokio::time::sleep(std::time::Duration::from_millis(EVENT_PACE_MS)).await;
        }
    });

    let entry_for_task = entry.clone();
    tokio::spawn(async move {
        let stats = run_streaming(state, allied, axis, |events, st| {
            let _ = ev_tx.send(WsMessage::Events { events });
            let sv = state_view(st);
            *entry_for_task.latest.lock() = Some(sv.clone());
            let _ = ev_tx.send(sv);
        })
        .await;
        let _ = ev_tx.send(WsMessage::Done {
            winner: stats.winner,
            turns: stats.turns,
            illegal_allied: stats.illegal_allied,
            illegal_axis: stats.illegal_axis,
        });
    });

    Json(json!({ "id": id }))
}

async fn list_games(State(app): State<AppState>) -> Json<Value> {
    let games = app.games.lock();
    let list: Vec<Value> = games
        .values()
        .map(|g| json!({ "id": g.id, "allied": g.allied, "axis": g.axis }))
        .collect();
    Json(json!(list))
}

async fn spectate_ws(
    ws: WebSocketUpgrade,
    State(app): State<AppState>,
    Path(id): Path<u64>,
) -> Response {
    ws.on_upgrade(move |mut socket| async move {
        let entry = app.games.lock().get(&id).cloned();
        match entry {
            Some(entry) => serve_game(socket, entry).await,
            None => {
                let msg = to_text(&WsMessage::Error {
                    message: format!("game {id} not found"),
                });
                let _ = socket.send(msg).await;
            }
        }
    })
}

async fn serve_game(mut socket: WebSocket, entry: Arc<GameEntry>) {
    if socket.send(to_text(&entry.snapshot)).await.is_err() {
        return;
    }
    let latest = entry.latest.lock().clone();
    if let Some(latest) = latest {
        if socket.send(to_text(&latest)).await.is_err() {
            return;
        }
    }
    let mut rx = entry.tx.subscribe();
    while let Ok(msg) = rx.recv().await {
        if socket.send(to_text(&msg)).await.is_err() {
            return;
        }
    }
}

// ---- replay ----

async fn list_replays() -> Json<Value> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir("games") {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".json") {
                out.push(json!({ "file": name }));
            }
        }
    }
    Json(json!(out))
}

async fn replay_ws(
    ws: WebSocketUpgrade,
    Path(file): Path<String>,
) -> Response {
    ws.on_upgrade(move |socket| async move { serve_replay(socket, file).await })
}

async fn serve_replay(mut socket: WebSocket, file: String) {
    let path: PathBuf = format!("games/{file}").into();
    let Ok(text) = std::fs::read_to_string(&path) else {
        let msg = to_text(&WsMessage::Error {
            message: format!("replay {file} not found"),
        });
        let _ = socket.send(msg).await;
        return;
    };
    let Ok(record): Result<Value, _> = serde_json::from_str(&text) else {
        let msg = to_text(&WsMessage::Error {
            message: "invalid replay file".to_string(),
        });
        let _ = socket.send(msg).await;
        return;
    };

    // Reconstruct the scenario map for the spectator (deterministic).
    let scenario = default_scenario();
    let snap = WsMessage::Snapshot {
        scenario: scenario.name.clone(),
        horizon: scenario.turn_horizon,
        map: scenario.map.clone(),
        objectives: scenario.map.objectives.clone(),
        depots: depots_of(&scenario.map),
        allied: record["allied"].as_str().unwrap_or("?").to_string(),
        axis: record["axis"].as_str().unwrap_or("?").to_string(),
        turn: 0,
        active: Side::Allied,
        winner: None,
        game_over: false,
        units: scenario.initial_units(),
    };
    if socket.send(to_text(&snap)).await.is_err() {
        return;
    }

    if let Some(events) = record["events"].as_array() {
        for ev in events {
            if let Ok(parsed) = serde_json::from_value::<cna_engine::Event>(ev.clone()) {
                let msg = to_text(&WsMessage::Events {
                    events: vec![parsed],
                });
                if socket.send(msg).await.is_err() {
                    return;
                }
            }
        }
    }

    let winner: Option<Side> = record["winner"]
        .as_str()
        .and_then(|s| serde_json::from_value(Value::String(s.to_string())).ok());
    let done = to_text(&WsMessage::Done {
        winner,
        turns: record["turns"].as_u64().unwrap_or(0) as u32,
        illegal_allied: record["illegal_allied"].as_u64().unwrap_or(0) as u32,
        illegal_axis: record["illegal_axis"].as_u64().unwrap_or(0) as u32,
    });
    let _ = socket.send(done).await;
}

// ---- static UI ----

async fn index() -> impl IntoResponse {
    match std::fs::read_to_string("web/dist/index.html") {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::OK,
            Html(
                "<h1>CNA AI Benchmark</h1><p>Web UI not built. Run <code>cd web && pnpm build</code>.</p>"
                    .to_string(),
            ),
        )
            .into_response(),
    }
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        games: Arc::new(Mutex::new(HashMap::new())),
        next_id: Arc::new(AtomicU64::new(1)),
        analyst: build_analyst(),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/health", get(health))
        .route("/api/games", get(list_games).post(start_game))
        .route("/api/replays", get(list_replays))
        .route("/api/analyze/:file", get(analyze))
        .route("/ws/spectate/:id", get(spectate_ws))
        .route("/ws/replay/:file", get(replay_ws))
        .nest_service("/assets", ServeDir::new("web/dist/assets"))
        .fallback_service(ServeFile::new("web/dist/index.html"))
        .with_state(app_state);

    let addr = std::env::var("CNA_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("CNA server listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
