import type { Hex } from './types';

const SQRT3 = Math.sqrt(3);

export function hexToPixel(q: number, r: number, size: number): { x: number; y: number } {
  return { x: size * SQRT3 * (q + r / 2), y: size * 1.5 * r };
}

export function hexCorners(cx: number, cy: number, size: number): [number, number][] {
  const pts: [number, number][] = [];
  for (let i = 0; i < 6; i++) {
    const angle = (Math.PI / 180) * (60 * i - 30);
    pts.push([cx + size * Math.cos(angle), cy + size * Math.sin(angle)]);
  }
  return pts;
}

export function hexPath(cx: number, cy: number, size: number): string {
  return (
    hexCorners(cx, cy, size)
      .map((p, i) => `${i === 0 ? 'M' : 'L'}${p[0].toFixed(2)},${p[1].toFixed(2)}`)
      .join(' ') + ' Z'
  );
}

export function mapBounds(hexes: Hex[], size: number) {
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  for (const h of hexes) {
    const { x, y } = hexToPixel(h.q, h.r, size);
    for (const [cx, cy] of hexCorners(x, y, size)) {
      minX = Math.min(minX, cx);
      maxX = Math.max(maxX, cx);
      minY = Math.min(minY, cy);
      maxY = Math.max(maxY, cy);
    }
  }
  return { minX, minY, width: maxX - minX, height: maxY - minY };
}
