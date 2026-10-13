import type { Unit } from '../types';

export default function UnitDetail({ unit, onClose }: { unit: Unit; onClose: () => void }) {
  const danger = (cond: boolean) => (cond ? ' danger' : '');
  return (
    <div className="unit-detail">
      <div className="unit-detail-head">
        <span className={`unit-detail-side ${unit.side === 'Allied' ? 'side-allied' : 'side-axis'}`}>{unit.side}</span>
        <span className="unit-detail-name">{unit.name}</span>
        <button className="unit-detail-close" onClick={onClose} title="Close">
          ✕
        </button>
      </div>
      <div className="unit-detail-sub">
        {unit.nationality} · {unit.kind}
      </div>
      <div className="unit-stat-grid">
        <div className="unit-stat">
          <span className="unit-stat-label">Strength</span>
          <span className="unit-stat-value">{unit.strength}/{unit.max_strength}</span>
        </div>
        <div className="unit-stat">
          <span className="unit-stat-label">Movement (CP)</span>
          <span className="unit-stat-value">{unit.movement}/{unit.movement_allowance}</span>
        </div>
        <div className="unit-stat">
          <span className="unit-stat-label">Supply</span>
          <span className={`unit-stat-value${danger(unit.supply === 'OutOfSupply')}`}>{unit.supply}</span>
        </div>
        <div className="unit-stat">
          <span className="unit-stat-label">Fuel</span>
          <span className={`unit-stat-value${danger(unit.fuel === 0 && unit.fuel_capacity > 0)}`}>
            {unit.fuel}/{unit.fuel_capacity}
          </span>
        </div>
        <div className="unit-stat">
          <span className="unit-stat-label">Ammo</span>
          <span className={`unit-stat-value${danger(unit.ammo === 0)}`}>{unit.ammo}/{unit.ammo_capacity}</span>
        </div>
        <div className="unit-stat">
          <span className="unit-stat-label">Water</span>
          <span className={`unit-stat-value${danger(unit.water === 0)}`}>{unit.water}/{unit.water_capacity}</span>
        </div>
        <div className="unit-stat">
          <span className="unit-stat-label">Sorties</span>
          <span className="unit-stat-value">{unit.missions}/{unit.missions_allowance}</span>
        </div>
        <div className="unit-stat">
          <span className="unit-stat-label">Attacked</span>
          <span className="unit-stat-value">{unit.has_attacked ? 'yes' : 'no'}</span>
        </div>
      </div>
    </div>
  );
}
