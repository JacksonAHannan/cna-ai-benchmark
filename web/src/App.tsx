import { useCallback, useEffect, useRef, useState } from 'react';
import type { AnalysisResponse, DepotView, Event, GameMap, Hex, Side, Unit, WsMessage } from './types';
import HexMap from './components/HexMap';
import EventFeed from './components/EventFeed';
import MatchReport from './components/MatchReport';

interface GameInfo {
  id: number;
  allied: string;
  axis: string;
}

interface ReplayInfo {
  file: string;
}

interface Spectate {
  scenario: string;
  horizon: number;
  map: GameMap | null;
  objectives: Hex[];
  depots: DepotView[];
  units: Unit[];
  events: Event[];
  turn: number;
  active: Side;
  winner: Side | null;
  gameOver: boolean;
  allied: string;
  axis: string;
  done: { winner: Side | null; turns: number; illegal_allied: number; illegal_axis: number } | null;
}

function applyEventsToUnits(units: Unit[], events: Event[]): Unit[] {
  let next = units;
  for (const ev of events) {
    if ((ev.type === 'move' || ev.type === 'retreat') && ev.unit != null && ev.to) {
      const unitId = ev.unit;
      const to = ev.to;
      next = next.map((u) => (u.id === unitId && u.hex ? { ...u, hex: to } : u));
    } else if (ev.type === 'elimination' && ev.unit != null) {
      const unitId = ev.unit;
      next = next.map((u) => (u.id === unitId ? { ...u, hex: null } : u));
    } else if (ev.type === 'reinforcement' && ev.unit != null && ev.hex) {
      const unitId = ev.unit;
      if (!next.some((u) => u.id === unitId)) {
        next = [
          ...next,
          {
            id: unitId,
            name: ev.name ?? '?',
            side: ev.side ?? 'Allied',
            nationality: ev.side === 'Axis' ? 'German' : 'British',
            kind: ev.kind ?? 'Infantry',
            strength: ev.strength ?? 1,
            max_strength: ev.strength ?? 1,
            movement: 0,
            movement_allowance: 0,
            fuel: 0,
            fuel_capacity: 0,
            supply: 'InSupply',
            missions: 0,
            missions_allowance: 0,
            hex: ev.hex,
            has_attacked: false,
          },
        ];
      }
    }
  }
  return next;
}

export default function App() {
  const [games, setGames] = useState<GameInfo[]>([]);
  const [replays, setReplays] = useState<ReplayInfo[]>([]);
  const [spec, setSpec] = useState<Spectate | null>(null);
  const [analysis, setAnalysis] = useState<AnalysisResponse | null>(null);
  const [mode, setMode] = useState('');
  const wsRef = useRef<WebSocket | null>(null);

  const refresh = useCallback(async () => {
    try {
      const [g, r] = await Promise.all([
        fetch('/api/games').then((res) => res.json()),
        fetch('/api/replays').then((res) => res.json()),
      ]);
      setGames(g);
      setReplays(r);
    } catch {
      /* server not up yet */
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const connect = useCallback((url: string) => {
    wsRef.current?.close();
    const ws = new WebSocket(url);
    wsRef.current = ws;
    ws.onmessage = (m) => {
      const msg = JSON.parse(m.data) as WsMessage;
      if (msg.type === 'snapshot') {
        setSpec({
          scenario: msg.scenario,
          horizon: msg.horizon,
          map: msg.map,
          objectives: msg.objectives,
          depots: msg.depots,
          units: msg.units,
          events: [],
          turn: msg.turn,
          active: msg.active,
          winner: msg.winner,
          gameOver: msg.game_over,
          allied: msg.allied,
          axis: msg.axis,
          done: null,
        });
      } else if (msg.type === 'events') {
        setSpec((s) =>
          s
            ? {
                ...s,
                events: [...s.events, ...msg.events],
                units: applyEventsToUnits(s.units, msg.events),
              }
            : s,
        );
      } else if (msg.type === 'state') {
        setSpec((s) =>
          s
            ? {
                ...s,
                units: msg.units,
                turn: msg.turn,
                active: msg.active,
                winner: msg.winner,
                gameOver: msg.game_over,
              }
            : s,
        );
      } else if (msg.type === 'done') {
        setSpec((s) =>
          s
            ? {
                ...s,
                gameOver: true,
                winner: msg.winner,
                done: {
                  winner: msg.winner,
                  turns: msg.turns,
                  illegal_allied: msg.illegal_allied,
                  illegal_axis: msg.illegal_axis,
                },
              }
            : s,
        );
      }
    };
  }, []);

  const startGame = async () => {
    const r = await fetch('/api/games', { method: 'POST' });
    const { id } = await r.json();
    setMode(`live #${id}`);
    setAnalysis(null);
    connect(`ws://${location.host}/ws/spectate/${id}`);
    refresh();
  };

  const spectate = (id: number) => {
    setMode(`live #${id}`);
    setAnalysis(null);
    connect(`ws://${location.host}/ws/spectate/${id}`);
  };

  const replay = (file: string) => {
    setMode(`replay ${file}`);
    connect(`ws://${location.host}/ws/replay/${file}`);
    fetch(`/api/analyze/${file}`)
      .then((r) => (r.ok ? r.json() : null))
      .then((a) => setAnalysis(a))
      .catch(() => setAnalysis(null));
  };

  return (
    <div className="app">
      <header className="header">
        <h1>Campaign for North Africa</h1>
        <span className="sub">AI BENCHMARK · SPECTATOR</span>
        <span className="spacer" />
        <button className="btn primary" style={{ width: 'auto' }} onClick={startGame}>
          ▶ Start new game
        </button>
      </header>

      <aside className="sidebar">
        <div className="panel">
          <h2>Live games</h2>
          {games.length === 0 && <div className="empty" style={{ padding: 8 }}>None running</div>}
          {games.map((g) => (
            <button className="btn" key={g.id} onClick={() => spectate(g.id)}>
              #{g.id} · {g.allied} vs {g.axis}
            </button>
          ))}
        </div>
        <div className="panel">
          <h2>Replays</h2>
          {replays.length === 0 && <div className="empty" style={{ padding: 8 }}>None yet</div>}
          {replays.map((r) => (
            <button className="btn" key={r.file} onClick={() => replay(r.file)}>
              {r.file}
            </button>
          ))}
        </div>
        <div className="panel">
          <h2>Legend</h2>
          <div className="legend">
            <span><span className="dot" style={{ background: 'var(--allied)' }} />Allied</span>
            <span><span className="dot" style={{ background: 'var(--axis)' }} />Axis</span>
            <span><span className="dot" style={{ background: 'var(--accent)' }} />Objective</span>
            <span><span className="dot" style={{ background: 'var(--sand)' }} />Desert</span>
            <span><span className="dot" style={{ background: 'var(--sea)' }} />Sea</span>
          </div>
        </div>
      </aside>

      <main className="main">
        <div className="map-wrap">
          <div className="statusbar">
            <span>Mode: <b>{mode || '—'}</b></span>
            {spec && (
              <>
                <span>Turn: <b>{spec.turn}/{spec.horizon}</b></span>
                <span>Active: <b>{spec.active}</b></span>
                <span>Allied: <b>{spec.allied}</b></span>
                <span>Axis: <b>{spec.axis}</b></span>
                {spec.done && (
                  <>
                    <span style={{ color: 'var(--accent)' }}>
                      Result: <b>{spec.done.winner ?? 'draw'}</b>
                    </span>
                    <span>Illegal: <b>{spec.done.illegal_allied}/{spec.done.illegal_axis}</b></span>
                  </>
                )}
              </>
            )}
          </div>
          {spec?.map ? (
            <HexMap map={spec.map} units={spec.units} depots={spec.depots} objectives={spec.objectives} />
          ) : (
            <div className="empty">Start or spectate a game to watch it unfold.</div>
          )}
        </div>
        {analysis && <MatchReport analysis={analysis} />}
        <EventFeed events={spec?.events ?? []} />
      </main>
    </div>
  );
}
