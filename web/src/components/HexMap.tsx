import type { DepotView, GameMap, Hex, MapHex, Unit } from '../types';
import { hexPath, hexToPixel, mapBounds } from '../layout';

const TERRAIN_COLOR: Record<string, string> = {
  Sea: 'var(--sea)',
  Desert: 'var(--sand)',
  Road: 'var(--road)',
  Coast: 'var(--coast)',
  Escarpment: 'var(--escarp)',
  Oasis: 'var(--oasis)',
  Village: 'var(--village)',
};

function siteName(map: GameMap, hex: Hex): string | null {
  const cell = map.hexes.find((h) => h.hex.q === hex.q && h.hex.r === hex.r);
  return cell?.name ?? null;
}

interface Props {
  map: GameMap;
  units: Unit[];
  depots: DepotView[];
  objectives: Hex[];
}

export default function HexMap({ map, units, depots, objectives }: Props) {
  const size = 20;
  const { minX, minY, width, height } = mapBounds(map.hexes.map((h) => h.hex), size);
  const pad = 26;

  return (
    <svg
      viewBox={`${minX - pad} ${minY - pad} ${width + pad * 2} ${height + pad * 2}`}
      preserveAspectRatio="xMidYMid meet"
    >
      {map.hexes.map((h: MapHex) => {
        const { x, y } = hexToPixel(h.hex.q, h.hex.r, size);
        return (
          <path
            key={`${h.hex.q},${h.hex.r}`}
            d={hexPath(x, y, size)}
            fill={TERRAIN_COLOR[h.terrain] ?? 'var(--sand)'}
            stroke="var(--border)"
            strokeWidth={0.7}
          />
        );
      })}

      {/* Objectives */}
      {objectives.map((o) => {
        const { x, y } = hexToPixel(o.q, o.r, size);
        const name = siteName(map, o);
        return (
          <g key={`obj-${o.q},${o.r}`}>
            <rect x={x - 5} y={y - 5} width={10} height={10} fill="var(--accent)" transform={`rotate(45 ${x} ${y})`} />
            <title>{name ?? 'objective'}</title>
            {name && (
              <text x={x} y={y - 12} textAnchor="middle" fontSize={8} fill="var(--accent)" fontFamily="monospace">
                {name}
              </text>
            )}
          </g>
        );
      })}

      {/* Depots */}
      {depots.map((d) => {
        const { x, y } = hexToPixel(d.hex.q, d.hex.r, size);
        const color = d.side === 'Allied' ? 'var(--allied)' : 'var(--axis)';
        return (
          <g key={`depot-${d.hex.q},${d.hex.r}`}>
            <circle cx={x} cy={y} r={6} fill={color} stroke="var(--panel)" strokeWidth={1.5} />
            <title>{d.name} (depot, {d.side})</title>
            <text x={x} y={y + 18} textAnchor="middle" fontSize={7.5} fill="var(--muted)" fontFamily="monospace">
              {d.name}
            </text>
          </g>
        );
      })}

      {/* Unit counters */}
      {units
        .filter((u) => u.hex)
        .map((u) => {
          const { x, y } = hexToPixel(u.hex!.q, u.hex!.r, size);
          const color = u.side === 'Allied' ? 'var(--allied)' : 'var(--axis)';
          return (
            <g key={`u-${u.id}`}>
              <circle cx={x} cy={y} r={11} fill="var(--panel-2)" stroke={color} strokeWidth={2} />
              <text x={x} y={y + 4} textAnchor="middle" fontSize={11} fontWeight="bold" fill="var(--text)" fontFamily="monospace">
                {u.strength}
              </text>
              <title>{`${u.name} (${u.side} ${u.nationality}, ${u.kind}, ${u.supply})`}</title>
            </g>
          );
        })}
    </svg>
  );
}
