import { useEffect, useMemo, useRef, useState } from 'react';
import type { DepotView, GameMap, Hex, Unit } from '../types';
import { hexPath, hexToPixel, mapBounds } from '../layout';

const TERRAIN_COLOR: Record<string, string> = {
  Sea: 'var(--sea)',
  Clear: 'var(--village)',
  Desert: 'var(--sand)',
  Rough: '#9a7a4e',
  Mountain: '#6a4a30',
  SaltMarsh: '#7a9a8a',
  RockGravel: '#9a9282',
  HeavyVegetation: '#3f6a34',
  Delta: '#2f5f5a',
  Swamp: '#5a6a3a',
  Escarpment: 'var(--escarp)',
  Road: 'var(--road)',
  Track: '#a08a58',
  Coast: 'var(--coast)',
  Oasis: 'var(--oasis)',
  Village: 'var(--village)',
  MajorCity: '#b0a080',
};

interface Props {
  map: GameMap;
  units: Unit[];
  depots: DepotView[];
  objectives: Hex[];
  selectedId: number | null;
  onSelect: (unit: Unit | null) => void;
  focusHex: { q: number; r: number } | null;
}

export default function HexMap({ map, units, depots, objectives, selectedId, onSelect, focusHex }: Props) {
  const size = 20;
  const pad = 26;
  const full = mapBounds(map.hexes.map((h) => h.hex), size);
  const totalW = full.width + pad * 2;
  const totalH = full.height + pad * 2;
  const MINIMAP_W = 220;
  const minimapH = (MINIMAP_W * totalH) / totalW;
  const minimapScale = MINIMAP_W / totalW;

  // Terrain is static; bake all hexes into one path per terrain type so the
  // map is 17 SVG nodes instead of ~20k and never re-renders on events.
  const terrainPaths = useMemo(() => {
    const main = new Map<string, string>();
    const mini = new Map<string, string>();
    for (const h of map.hexes) {
      const { x, y } = hexToPixel(h.hex.q, h.hex.r, size);
      main.set(h.terrain, (main.get(h.terrain) ?? '') + hexPath(x, y, size));
      const mx = (x - full.minX) * minimapScale;
      const my = (y - full.minY) * minimapScale;
      mini.set(h.terrain, (mini.get(h.terrain) ?? '') + hexPath(mx, my, size * minimapScale));
    }
    return { main, mini };
  }, [map, size, full.minX, full.minY, minimapScale]);

  const [zoom, setZoom] = useState(1);
  const [center, setCenter] = useState({ x: full.minX + full.width / 2, y: full.minY + full.height / 2 });
  const [follow, setFollow] = useState(true);
  const svgRef = useRef<SVGSVGElement>(null);
  const dragRef = useRef<{ sx: number; sy: number; cx: number; cy: number } | null>(null);

  useEffect(() => {
    if (!follow || !focusHex) return;
    const { x, y } = hexToPixel(focusHex.q, focusHex.r, size);
    setCenter({ x, y });
  }, [focusHex, follow]);

  const winW = totalW / zoom;
  const winH = totalH / zoom;
  const viewBox = `${center.x - winW / 2} ${center.y - winH / 2} ${winW} ${winH}`;

  const applyZoom = (factor: number) => setZoom((z) => Math.min(8, Math.max(1, z * factor)));

  const onWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    applyZoom(e.deltaY < 0 ? 1.2 : 1 / 1.2);
  };

  const scale = svgRef.current ? winW / svgRef.current.clientWidth : 1;

  return (
    <div className="map-container">
      <svg
        ref={svgRef}
        viewBox={viewBox}
        preserveAspectRatio="xMidYMid meet"
        onWheel={onWheel}
        onMouseDown={(e) => {
          dragRef.current = { sx: e.clientX, sy: e.clientY, cx: center.x, cy: center.y };
          onSelect(null);
        }}
        onMouseMove={(e) => {
          if (!dragRef.current) return;
          const dx = (e.clientX - dragRef.current.sx) * scale;
          const dy = (e.clientY - dragRef.current.sy) * scale;
          setCenter({ x: dragRef.current.cx - dx, y: dragRef.current.cy - dy });
        }}
        onMouseUp={() => (dragRef.current = null)}
        onMouseLeave={() => (dragRef.current = null)}
        style={{ cursor: dragRef.current ? 'grabbing' : 'grab' }}
      >
        {[...terrainPaths.main.entries()].map(([terrain, d]) => (
          <path
            key={terrain}
            d={d}
            fill={TERRAIN_COLOR[terrain] ?? 'var(--sand)'}
            stroke="var(--border)"
            strokeWidth={0.7}
          />
        ))}

        {/* Objectives */}
        {objectives.map((o) => {
          const { x, y } = hexToPixel(o.q, o.r, size);
          const name = map.hexes.find((h) => h.hex.q === o.q && h.hex.r === o.r)?.name ?? null;
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

        {/* Unit counters — CSS-transformed so moves animate between hexes */}
        {units
          .filter((u) => u.hex)
          .map((u) => {
            const { x, y } = hexToPixel(u.hex!.q, u.hex!.r, size);
            const color = u.side === 'Allied' ? 'var(--allied)' : 'var(--axis)';
            const selected = u.id === selectedId;
            return (
              <g
                key={u.id}
                className={`unit${selected ? ' unit-selected' : ''}`}
                style={{
                  transform: `translate(${x}px, ${y}px)`,
                  transition: 'transform 600ms cubic-bezier(0.35, 0, 0.2, 1)',
                  cursor: 'pointer',
                }}
                onClick={(e) => {
                  e.stopPropagation();
                  onSelect(u);
                }}
                onMouseDown={(e) => e.stopPropagation()}
              >
                <circle
                  cx={0}
                  cy={0}
                  r={11}
                  fill="var(--panel-2)"
                  stroke={u.supply === 'OutOfSupply' ? 'var(--error)' : color}
                  strokeWidth={selected ? 3 : 2}
                />
                <text x={0} y={4} textAnchor="middle" fontSize={11} fontWeight="bold" fill="var(--text)" fontFamily="monospace">
                  {u.strength}
                </text>
                {(u.water === 0 || (u.fuel === 0 && u.fuel_capacity > 0) || u.contact) && (
                  <g>
                    {u.water === 0 && <circle cx={-4} cy={-8} r={2.5} fill="#4a8ad0" />}
                    {u.fuel === 0 && u.fuel_capacity > 0 && <circle cx={4} cy={-8} r={2.5} fill="var(--warning)" />}
                    {u.contact && <circle cx={0} cy={-8} r={2.5} fill="var(--error)" />}
                  </g>
                )}
                <title>{`${u.name} (${u.side} ${u.nationality}, ${u.kind}, ${u.supply})`}</title>
              </g>
            );
          })}
      </svg>

      <svg
        className="minimap"
        width={MINIMAP_W}
        height={minimapH}
        viewBox={`0 0 ${MINIMAP_W} ${minimapH}`}
        style={{ width: MINIMAP_W, height: minimapH }}
        onClick={(e) => {
          const rect = (e.currentTarget as SVGSVGElement).getBoundingClientRect();
          setCenter({
            x: ((e.clientX - rect.left) / rect.width) * totalW + full.minX,
            y: ((e.clientY - rect.top) / rect.height) * totalH + full.minY,
          });
          setFollow(false);
        }}
      >
        <rect x={0} y={0} width={MINIMAP_W} height={minimapH} fill="var(--sea)" />
        {[...terrainPaths.mini.entries()].map(([terrain, d]) => (
          <path
            key={`mm-${terrain}`}
            d={d}
            fill={TERRAIN_COLOR[terrain] ?? 'var(--sand)'}
            stroke="none"
          />
        ))}
        <rect
          x={(center.x - winW / 2 - full.minX) * minimapScale}
          y={(center.y - winH / 2 - full.minY) * minimapScale}
          width={winW * minimapScale}
          height={winH * minimapScale}
          fill="none"
          stroke="var(--accent)"
          strokeWidth={1.5}
        />
      </svg>

      <div className="map-controls">
        <button
          onClick={() => setFollow((f) => !f)}
          style={follow ? { borderColor: 'var(--accent)', color: 'var(--accent)' } : {}}
          title={follow ? 'Following action (click to stop)' : 'Follow action'}
        >
          ◎
        </button>
        <button onClick={() => applyZoom(1.35)} title="Zoom in">＋</button>
        <button onClick={() => applyZoom(1 / 1.35)} title="Zoom out">－</button>
        <button
          onClick={() => {
            setZoom(1);
            setCenter({ x: full.minX + full.width / 2, y: full.minY + full.height / 2 });
          }}
          title="Reset view"
        >
          ⟳
        </button>
      </div>
    </div>
  );
}
