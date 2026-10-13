import { useEffect, useRef } from 'react';
import type { Event, GameMap, Unit } from '../types';

interface Resolver {
  unitName: (id?: number) => string;
  placeName: (h?: { q: number; r: number }) => string;
}

function describe(ev: Event, r: Resolver): { text: string; cls: string } {
  switch (ev.type) {
    case 'gameStart':
      return { text: `⚑ Campaign begins (scenario ${String(ev.turn_horizon ?? '')} weeks)`, cls: 'evt-gameOver' };
    case 'turnStart':
      return { text: `▸ Turn ${ev.turn} — ${ev.side} command`, cls: 'evt-turnStart' };
    case 'supply':
      return { text: `supply: ${ev.name} → ${ev.status}`, cls: 'evt-supply' };
    case 'convoy':
      return { text: `🚢 Axis convoy: ${ev.tons} tons`, cls: 'evt-supply' };
    case 'evaporation':
      return { text: `evaporation: ${ev.subject} (-${ev.amount} fuel @${ev.rate_percent}%)`, cls: 'evt-evaporation' };
    case 'pasta':
      return { text: `🍝 pasta: ${ev.name} — ${ev.note}`, cls: 'evt-pasta' };
    case 'move':
      return { text: `move: ${r.unitName(ev.unit)} ${r.placeName(ev.from)} → ${r.placeName(ev.to)}`, cls: 'evt-move' };
    case 'attack':
      return {
        text: `⚔ ${r.unitName(ev.attacker)} attacks ${r.unitName(ev.defender)} (${ev.odds_numerator}:${ev.odds_denominator}) → ${ev.result}`,
        cls: 'evt-attack',
      };
    case 'retreat':
      return { text: `↩ ${r.unitName(ev.unit)} retreats ${r.placeName(ev.from)} → ${r.placeName(ev.to)}`, cls: 'evt-retreat' };
    case 'elimination':
      return { text: `✕ ${ev.name} destroyed`, cls: 'evt-elimination' };
    case 'airRecon':
      return {
        text: `✈ recon: ${r.unitName(ev.squadron)} over ${r.placeName(ev.target)} (revealed ${(ev.revealed ?? []).length})`,
        cls: 'evt-airRecon',
      };
    case 'bomb':
      return {
        text: `💣 bombing: ${r.unitName(ev.squadron)} → ${r.placeName(ev.target)} (supply −${ev.supply_hit ?? 0}${ev.intercepted ? ', intercepted' : ''})`,
        cls: 'evt-airRecon',
      };
    case 'airCombat':
      return {
        text: `✈ air combat: ${r.unitName(ev.attacker)} vs ${r.unitName(ev.defender)} (losses ${ev.attacker_losses ?? 0}/${ev.defender_losses ?? 0})`,
        cls: 'evt-attack',
      };
    case 'reinforcement':
      return {
        text: `+ ${ev.name ?? r.unitName(ev.unit)} arrives at ${r.placeName(ev.hex)}`,
        cls: 'evt-reinforcement',
      };
    case 'illegalAction':
      return { text: `⚠ illegal action (${ev.side}): ${ev.error}`, cls: 'evt-illegalAction' };
    case 'turnEnd':
      return { text: `■ ${ev.side} ends turn`, cls: 'evt-turnStart' };
    case 'victory':
      return { text: `★ ${ev.side} VICTORY (${ev.reason})`, cls: 'evt-victory' };
    case 'gameOver':
      return {
        text: `🏁 Game over — ${ev.outcome?.winner ? ev.outcome.winner : 'draw'} (Allied ${ev.outcome?.allied_score} : Axis ${ev.outcome?.axis_score})`,
        cls: 'evt-gameOver',
      };
    default:
      return { text: `${ev.type}`, cls: 'evt-move' };
  }
}

export default function EventFeed({
  events,
  units,
  map,
}: {
  events: Event[];
  units: Unit[];
  map: GameMap | null;
}) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (ref.current) {
      ref.current.scrollTop = ref.current.scrollHeight;
    }
  }, [events.length]);

  const resolver: Resolver = {
    unitName: (id) => {
      if (id == null) return '?';
      return units.find((u) => u.id === id)?.name ?? `unit ${id}`;
    },
    placeName: (h) => {
      if (!h) return '?';
      const mh = map?.hexes.find((x) => x.hex.q === h.q && x.hex.r === h.r);
      return mh?.name ?? `${h.q},${h.r}`;
    },
  };

  return (
    <div className="feed" ref={ref}>
      {events.length === 0 && <div className="empty">Awaiting events…</div>}
      {events.map((ev, i) => {
        const d = describe(ev, resolver);
        return (
          <div className={`line ${d.cls}`} key={i}>
            <span className="tag">{ev.turn ? `T${ev.turn}` : '—'}</span>
            {d.text}
          </div>
        );
      })}
    </div>
  );
}
