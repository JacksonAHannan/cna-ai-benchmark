import { useEffect, useRef } from 'react';
import type { Event } from '../types';

function hexLabel(h?: { q: number; r: number }): string {
  return h ? `${h.q},${h.r}` : '?';
}

function describe(ev: Event): { text: string; cls: string } {
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
      return { text: `move: unit ${ev.unit} ${hexLabel(ev.from)} → ${hexLabel(ev.to)}`, cls: 'evt-move' };
    case 'attack':
      return { text: `⚔ attack: ${ev.attacker} vs ${ev.defender} (${ev.odds_numerator}:${ev.odds_denominator}) → ${ev.result}`, cls: 'evt-attack' };
    case 'retreat':
      return { text: `↩ retreat: unit ${ev.unit} ${hexLabel(ev.from)} → ${hexLabel(ev.to)}`, cls: 'evt-retreat' };
    case 'elimination':
      return { text: `✕ ${ev.name} destroyed`, cls: 'evt-elimination' };
    case 'airRecon':
      return { text: `✈ recon: squadron ${ev.squadron} over ${hexLabel(ev.target)} (revealed ${(ev.revealed ?? []).length})`, cls: 'evt-airRecon' };
    case 'bomb':
      return {
        text: `💣 bombing: squadron ${ev.squadron} → ${hexLabel(ev.target)} (supply −${ev.supply_hit ?? 0}${ev.intercepted ? ', intercepted' : ''})`,
        cls: 'evt-airRecon',
      };
    case 'airCombat':
      return {
        text: `✈ air combat: ${ev.attacker} vs ${ev.defender} (losses ${ev.attacker_losses ?? 0}/${ev.defender_losses ?? 0})`,
        cls: 'evt-attack',
      };
    case 'reinforcement':
      return { text: `+ reinforcement: ${ev.name} arrives at ${hexLabel(ev.hex)}`, cls: 'evt-reinforcement' };
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

export default function EventFeed({ events }: { events: Event[] }) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (ref.current) {
      ref.current.scrollTop = ref.current.scrollHeight;
    }
  }, [events.length]);

  return (
    <div className="feed" ref={ref}>
      {events.length === 0 && <div className="empty">Awaiting events…</div>}
      {events.map((ev, i) => {
        const d = describe(ev);
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
