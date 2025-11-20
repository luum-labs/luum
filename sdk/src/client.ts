import { Connection, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { WalletFlow, SankeyGraph, AnalysisOptions, FlowCategory, TierInfo, TIER_NAMES } from "./types";

export class LuumClient {
  private connection: Connection;

  constructor(rpcUrl: string) {
    this.connection = new Connection(rpcUrl, "confirmed");
  }

  async analyzeWallet(
    address: string,
    options: AnalysisOptions = {}
  ): Promise<{ flows: WalletFlow[]; sankey: SankeyGraph }> {
    const pubkey = new PublicKey(address);
    const days = options.days ?? 7;
    const minAmount = options.minAmount ?? 100;

    const signatures = await this.connection.getSignaturesForAddress(pubkey, {
      limit: 1000,
    });

    const cutoff = Date.now() - days * 86_400_000;
    const recent = signatures.filter(
      (s) => s.blockTime && s.blockTime * 1000 >= cutoff
    );

    const flowMap = new Map<string, { count: number; total: BN }>();

    for (const sig of recent) {
      const tx = await this.connection.getParsedTransaction(sig.signature, {
        maxSupportedTransactionVersion: 0,
      });
      if (!tx?.meta) continue;

      const pre = tx.meta.preTokenBalances ?? [];
      const post = tx.meta.postTokenBalances ?? [];

      for (let i = 0; i < post.length; i++) {
        const preAmt = pre[i]?.uiTokenAmount?.amount ?? "0";
        const postAmt = post[i]?.uiTokenAmount?.amount ?? "0";
        const diff = new BN(postAmt).sub(new BN(preAmt));

        if (diff.gtn(minAmount)) {
          const owner = post[i].owner ?? "unknown";
          const existing = flowMap.get(owner) ?? { count: 0, total: new BN(0) };
          existing.count += 1;
          existing.total = existing.total.add(diff);
          flowMap.set(owner, existing);
        }
      }
    }

    const flows: WalletFlow[] = Array.from(flowMap.entries())
      .map(([addr, data]) => ({
        address: addr,
        txCount: data.count,
        totalAmount: data.total,
        category: this.categorize(data.count, data.total),
        riskScore: this.computeRisk(data.count, data.total),
      }))
      .sort((a, b) => b.totalAmount.cmp(a.totalAmount));

    const sankey = this.buildSankey(address, flows);
    return { flows, sankey };
  }

  async getTierInfo(balance: BN, totalSupply: BN): Promise<TierInfo> {
    if (totalSupply.isZero()) {
      return { level: 0, name: "Free", maxQueries: 3, maxAgents: 1 };
    }
    const ratioBps = balance.mul(new BN(10_000)).div(totalSupply).toNumber();
    let level = 0;
    if (ratioBps >= 10) level = 4;
    else if (ratioBps >= 5) level = 3;
    else if (ratioBps >= 1) level = 2;
    else if (ratioBps > 0) level = 1;

    const queryLimits = [3, 50, -1, -1, -1];
    const agentLimits = [1, 3, -1, -1, -1];

    return {
      level,
      name: TIER_NAMES[level],
      maxQueries: queryLimits[level],
      maxAgents: agentLimits[level],
    };
  }

  private categorize(txCount: number, total: BN): FlowCategory {
    const avg = txCount > 0 ? total.divn(txCount).toNumber() : 0;
    if (txCount > 1000 && avg < 10_000) return FlowCategory.ApiProvider;
    if (txCount > 500 && avg < 50_000) return FlowCategory.Oracle;
    if (txCount < 10 && total.gtn(1_000_000)) return FlowCategory.Exchange;
    if (txCount > 200) return FlowCategory.Compute;
    return FlowCategory.Unknown;
  }

  private computeRisk(txCount: number, total: BN): number {
    const freq = Math.min(Math.log(txCount + 1) / Math.log(1000), 1);
    const amt = Math.min(Math.log(total.toNumber() + 1) / Math.log(10_000_000), 1);
    return Math.round((freq * 0.4 + amt * 0.6) * 100);
  }

  private buildSankey(source: string, flows: WalletFlow[]): SankeyGraph {
    const srcLabel = source.length > 8 ? source.slice(0, 4) + "..." + source.slice(-4) : source;
    const nodes = [{ id: "src", label: srcLabel, value: 0, depth: 0 }];
    const links = [];
    let totalFlow = 0;

    for (const flow of flows) {
      const id = "n_" + flow.address.slice(0, 8);
      const label = flow.address.slice(0, 4) + "..." + flow.address.slice(-4);
      const val = flow.totalAmount.toNumber();
      nodes.push({ id, label, value: val, depth: 1 });
      links.push({ source: "src", target: id, value: val, category: flow.category });
      totalFlow += val;
    }

    nodes[0].value = totalFlow;
    return { nodes, links, totalFlow };
  }
}
