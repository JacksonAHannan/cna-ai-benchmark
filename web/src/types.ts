export type Side = 'Allied' | 'Axis';
export type Terrain = 'Sea' | 'Desert' | 'Road' | 'Coast' | 'Escarpment' | 'Oasis' | 'Village';

export interface Hex {
  q: number;
  r: number;
}

export type Site =
  | { kind: 'depot'; side: Side; name: string; capacity: number }
  | { kind: 'objective'; name: string };

export interface MapHex {
  hex: Hex;
  terrain: Terrain;
  name: string | null;
  site: Site | null;
}

export interface GameMap {
  rows: number;
  cols: number;
  hexes: MapHex[];
  depots: Hex[];
  objectives: Hex[];
}

export interface DepotView {
  hex: Hex;
  name: string;
  side: Side;
  capacity: number;
}

export interface Unit {
  id: number;
  name: string;
  side: Side;
  nationality: string;
  kind: string;
  strength: number;
  max_strength: number;
  movement: number;
  movement_allowance: number;
  fuel: number;
  fuel_capacity: number;
  supply: string;
  missions: number;
  missions_allowance: number;
  hex: Hex | null;
  has_attacked: boolean;
}

export interface Outcome {
  winner: Side | null;
  reason: string;
  allied_score: number;
  axis_score: number;
}

export interface Event {
  type: string;
  turn?: number;
  side?: Side;
  unit?: number;
  name?: string;
  from?: Hex;
  to?: Hex;
  attacker?: number;
  defender?: number;
  target?: Hex;
  squadron?: number;
  revealed?: number[];
  result?: string;
  reason?: string;
  amount?: number;
  rate_percent?: number;
  status?: string;
  error?: string;
  note?: string;
  subject?: string;
  hex?: Hex;
  kind?: string;
  strength?: number;
  turn_horizon?: number;
  odds_numerator?: number;
  odds_denominator?: number;
  outcome?: Outcome;
}

export type WsMessage =
  | {
      type: 'snapshot';
      scenario: string;
      horizon: number;
      map: GameMap;
      objectives: Hex[];
      depots: DepotView[];
      allied: string;
      axis: string;
      turn: number;
      active: Side;
      winner: Side | null;
      game_over: boolean;
      units: Unit[];
    }
  | { type: 'events'; events: Event[] }
  | { type: 'state'; units: Unit[]; turn: number; active: Side; winner: Side | null; game_over: boolean }
  | { type: 'done'; winner: Side | null; turns: number; illegal_allied: number; illegal_axis: number }
  | { type: 'error'; message: string };

export interface GameSummary {
  winner: Side | null;
  reason: string;
  turns: number;
  allied_score: number;
  axis_score: number;
  illegal_allied: number;
  illegal_axis: number;
  moves: number;
  attacks: number;
  air_recons: number;
  supply_checks: number;
  out_of_supply: number;
  evaporation: number;
  pasta: number;
  allied_eliminations: string[];
  axis_eliminations: string[];
  reinforcements: string[];
  attack_results: string[];
  timeline: string[];
}

export interface AnalysisResponse {
  allied: string;
  axis: string;
  summary: GameSummary;
  narrative: string;
}
