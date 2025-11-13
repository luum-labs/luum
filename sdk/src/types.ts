import BN from "bn.js";

export interface WalletFlow {
  address: string;
  txCount: number;
  totalAmount: BN;
  category: FlowCategory;
  riskScore: number;
}

export enum FlowCategory {
  ApiProvider = "ApiProvider",
  Oracle = "Oracle",
  Exchange = "Exchange",
  Compute = "Compute",
  DataFeed = "DataFeed",
  Unknown = "Unknown",
}

export interface SankeyNode {
  id: string;
  label: string;
  value: number;
  depth: number;
}

export interface SankeyLink {
  source: string;
  target: string;
  value: number;
  category: string;
}

export interface SankeyGraph {
  nodes: SankeyNode[];
  links: SankeyLink[];
  totalFlow: number;
}

export interface AnalysisOptions {
  days?: number;
  minAmount?: number;
  includeInbound?: boolean;
}

export interface TierInfo {
  level: number;
  name: string;
  maxQueries: number;
  maxAgents: number;
}

export const TIER_NAMES: Record<number, string> = {
  0: "Free",
  1: "Basic",
  2: "Pro",
  3: "Elite",
  4: "Whale",
};
