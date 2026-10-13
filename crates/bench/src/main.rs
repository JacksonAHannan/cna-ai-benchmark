//! Headless benchmark runner: plays a round-robin of games between agents,
//! updates Elo, and persists each game's event log for later spectating/replay.

use cna_engine::{default_scenario, Event, GameState, Side};
use cna_models::{build_agent, run_streaming, run_streaming_team, Agent, AgentConfig};
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

const DEFAULT_GAMES_PER_PAIR: usize = 1;
const DEFAULT_SEED: u64 = 42;

#[derive(Debug, Serialize)]
struct GameRecord {
    seed: u64,
    allied: String,
    axis: String,
    winner: Option<Side>,
    turns: u32,
    illegal_allied: u32,
    illegal_axis: u32,
    events: Vec<Event>,
}

#[derive(Debug)]
struct GameResult {
    winner: Option<Side>,
    turns: u32,
    illegal_allied: u32,
    illegal_axis: u32,
    events: Vec<Event>,
}

async fn run_game(
    allied: Arc<dyn Agent>,
    axis: Arc<dyn Agent>,
    seed: u64,
    turns: Option<u32>,
) -> GameResult {
    let mut scenario = default_scenario();
    if let Some(t) = turns {
        scenario.turn_horizon = t;
    }
    let state = GameState::new(scenario, seed);
    let mut events = Vec::new();
    let stats = run_streaming(state, allied, axis, |ev, _st| events.extend(ev)).await;
    GameResult {
        winner: stats.winner,
        turns: stats.turns,
        illegal_allied: stats.illegal_allied,
        illegal_axis: stats.illegal_axis,
        events,
    }
}

async fn run_team_game(
    team_a: [Arc<dyn Agent>; 5],
    team_b: [Arc<dyn Agent>; 5],
    seed: u64,
    turns: Option<u32>,
) -> GameResult {
    let mut scenario = default_scenario();
    if let Some(t) = turns {
        scenario.turn_horizon = t;
    }
    let state = GameState::new(scenario, seed);
    let mut events = Vec::new();
    let stats = run_streaming_team(state, team_a, team_b, |ev, _st| events.extend(ev)).await;
    GameResult {
        winner: stats.winner,
        turns: stats.turns,
        illegal_allied: stats.illegal_allied,
        illegal_axis: stats.illegal_axis,
        events,
    }
}

fn expected(ra: f64, rb: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((rb - ra) / 400.0))
}

fn update_elo(ra: f64, rb: f64, score_a: f64, k: f64) -> (f64, f64) {
    let ea = expected(ra, rb);
    (ra + k * (score_a - ea), rb + k * (1.0 - score_a - (1.0 - ea)))
}

fn outcome_score(winner: Option<Side>) -> (f64, f64) {
    match winner {
        Some(Side::Allied) => (1.0, 0.0),
        Some(Side::Axis) => (0.0, 1.0),
        None => (0.5, 0.5),
    }
}

fn parse_args() -> (usize, u64, Option<PathBuf>, Option<u32>, bool) {
    let mut games = DEFAULT_GAMES_PER_PAIR;
    let mut seed = DEFAULT_SEED;
    let mut config: Option<PathBuf> = None;
    let mut turns: Option<u32> = None;
    let mut teams = false;

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--games" => {
                if let Some(v) = args.next() {
                    games = v.parse().unwrap_or(DEFAULT_GAMES_PER_PAIR);
                }
            }
            "--seed" => {
                if let Some(v) = args.next() {
                    seed = v.parse().unwrap_or(DEFAULT_SEED);
                }
            }
            "--config" => {
                config = args.next().map(PathBuf::from);
            }
            "--turns" => {
                turns = args.next().and_then(|v| v.parse().ok());
            }
            "--teams" => {
                teams = true;
            }
            _ => {}
        }
    }
    (games, seed, config, turns, teams)
}

fn load_agents(config: Option<PathBuf>) -> Vec<Arc<dyn Agent>> {
    if let Some(path) = config {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let configs: Vec<AgentConfig> = serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("invalid agent config {}: {e}", path.display()));
        configs.iter().map(|c| Arc::from(build_agent(c))).collect()
    } else {
        vec![
            Arc::from(build_agent(&AgentConfig::mock("mock-a"))),
            Arc::from(build_agent(&AgentConfig::mock("mock-b"))),
        ]
    }
}

#[tokio::main]
async fn main() {
    let (games_per_pair, seed_base, config, turns, teams) = parse_args();

    if teams {
        // 5v5 smoke run: five mock roles per side.
        let team_a: [Arc<dyn Agent>; 5] = std::array::from_fn(|i| {
            Arc::from(build_agent(&AgentConfig::mock(&format!("mock-a-{i}"))))
        });
        let team_b: [Arc<dyn Agent>; 5] = std::array::from_fn(|i| {
            Arc::from(build_agent(&AgentConfig::mock(&format!("mock-b-{i}"))))
        });
        let r = run_team_game(team_a, team_b, seed_base, turns).await;
        println!(
            "5v5: winner={:?} turns={} illegal_allied={} illegal_axis={}",
            r.winner, r.turns, r.illegal_allied, r.illegal_axis
        );
        return;
    }

    let players = load_agents(config);

    if players.len() < 2 {
        eprintln!("need at least 2 agents");
        std::process::exit(1);
    }

    // Build the schedule and spawn all games concurrently.
    let mut schedule: Vec<(usize, usize, u64)> = Vec::new();
    let mut seed = seed_base;
    for i in 0..players.len() {
        for j in (i + 1)..players.len() {
            for g in 0..games_per_pair {
                let (ai, bi) = if g % 2 == 0 { (i, j) } else { (j, i) };
                schedule.push((ai, bi, seed));
                seed = seed.wrapping_add(1);
            }
        }
    }

    let handles: Vec<_> = schedule
        .iter()
        .map(|&(ai, bi, s)| {
            let allied = players[ai].clone();
            let axis = players[bi].clone();
            let an = players[ai].name().to_string();
            let bn = players[bi].name().to_string();
            tokio::spawn(async move {
                let r = run_game(allied, axis, s, turns).await;
                (s, an, bn, r)
            })
        })
        .collect();

    // Apply Elo in schedule order (deterministic).
    let mut elo: Vec<f64> = vec![1500.0; players.len()];
    let k = 32.0;
    std::fs::create_dir_all("games").ok();

    for (idx, handle) in handles.into_iter().enumerate() {
        let (s, an, bn, r) = handle.await.expect("game task panicked");
        let (ai, bi, _) = schedule[idx];
        let (sa, _sb) = outcome_score(r.winner);
        let (na, nb) = update_elo(elo[ai], elo[bi], sa, k);
        elo[ai] = na;
        elo[bi] = nb;

        let record = GameRecord {
            seed: s,
            allied: an.clone(),
            axis: bn.clone(),
            winner: r.winner,
            turns: r.turns,
            illegal_allied: r.illegal_allied,
            illegal_axis: r.illegal_axis,
            events: r.events,
        };
        let path = format!("games/game_{s}.json");
        if let Ok(json) = serde_json::to_string_pretty(&record) {
            std::fs::write(&path, json).ok();
        }

        let winner = r
            .winner
            .map(|w| w.to_string())
            .unwrap_or_else(|| "draw".to_string());
        println!(
            "[{s:04}] {an:>12} vs {bn:<12} -> {winner:8} (turns {}, illegal {}/{} -> elo {:.0}/{:.0})",
            r.turns, r.illegal_allied, r.illegal_axis, elo[ai], elo[bi]
        );
    }

    // Final ranking.
    let mut rank: Vec<(usize, f64)> = (0..players.len()).map(|i| (i, elo[i])).collect();
    rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("\n=== Elo ranking ===");
    for (i, (idx, rating)) in rank.iter().enumerate() {
        println!("{}. {:>12}  {:.0}", i + 1, players[*idx].name(), rating);
    }
}
