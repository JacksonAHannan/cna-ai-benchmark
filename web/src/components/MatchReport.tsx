import type { AnalysisResponse } from '../types';

function Stat({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="stat">
      <span className="stat-label">{label}</span>
      <span className="stat-value">{value}</span>
    </div>
  );
}

export default function MatchReport({ analysis }: { analysis: AnalysisResponse }) {
  const s = analysis.summary;
  return (
    <div className="report">
      <div className="report-head">
        <span className="report-title">Match report</span>
        <span className="report-match">
          {analysis.allied} vs {analysis.axis}
        </span>
        <span className={`report-winner ${s.winner ? `side-${s.winner.toLowerCase()}` : ''}`}>
          {s.winner ? `${s.winner} wins` : 'Draw'} · {s.reason}
        </span>
      </div>

      <div className="stats-grid">
        <Stat label="Turns" value={s.turns} />
        <Stat label="Allied score" value={s.allied_score} />
        <Stat label="Axis score" value={s.axis_score} />
        <Stat label="Moves" value={s.moves} />
        <Stat label="Attacks" value={s.attacks} />
        <Stat label="Air recon" value={s.air_recons} />
        <Stat label="Supply checks" value={s.supply_checks} />
        <Stat label="Out of supply" value={s.out_of_supply} />
        <Stat label="Evaporation" value={s.evaporation} />
        <Stat label="Pasta" value={s.pasta} />
        <Stat label="Illegal (Allied)" value={s.illegal_allied} />
        <Stat label="Illegal (Axis)" value={s.illegal_axis} />
        <Stat label="Bombs" value={s.bombs} />
        <Stat label="Air combats" value={s.air_combats} />
        <Stat label="Axis convoy tons" value={s.convoy_tons} />
      </div>

      {(s.allied_eliminations.length > 0 || s.axis_eliminations.length > 0) && (
        <div className="report-section">
          <h3>Casualties</h3>
          <div className="report-lists">
            <div>
              <b className="side-allied">Allied:</b>{' '}
              {s.allied_eliminations.join(', ') || '—'}
            </div>
            <div>
              <b className="side-axis">Axis:</b> {s.axis_eliminations.join(', ') || '—'}
            </div>
          </div>
        </div>
      )}

      {s.reinforcements.length > 0 && (
        <div className="report-section">
          <h3>Reinforcements</h3>
          <div>{s.reinforcements.join(', ')}</div>
        </div>
      )}

      <div className="report-section">
        <h3>Strategy &amp; decision-making</h3>
        <p className="narrative">{analysis.narrative}</p>
      </div>

      <div className="report-section">
        <h3>Timeline</h3>
        <div className="timeline">
          {s.timeline.map((t, i) => (
            <div key={i}>{t}</div>
          ))}
        </div>
      </div>
    </div>
  );
}
