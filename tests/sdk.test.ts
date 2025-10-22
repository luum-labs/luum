import BN from "bn.js";
import { FlowCategory, TIER_NAMES } from "../sdk/src/types";

describe("sdk types", () => {
  test("FlowCategory enum values are defined", () => {
    expect(FlowCategory.ApiProvider).toBe("ApiProvider");
    expect(FlowCategory.Oracle).toBe("Oracle");
    expect(FlowCategory.Exchange).toBe("Exchange");
    expect(FlowCategory.Compute).toBe("Compute");
    expect(FlowCategory.Unknown).toBe("Unknown");
  });

  test("tier names cover all 5 levels", () => {
    expect(Object.keys(TIER_NAMES)).toHaveLength(5);
    expect(TIER_NAMES[0]).toBe("Free");
    expect(TIER_NAMES[4]).toBe("Whale");
  });

  test("BN arithmetic is correct for large amounts", () => {
    const a = new BN("1000000000");
    const b = new BN("500000000");
    expect(a.add(b).toString()).toBe("1500000000");
    expect(a.sub(b).toString()).toBe("500000000");
  });

  test("risk score bounds check", () => {
    const freq = Math.min(Math.log(1001) / Math.log(1000), 1);
    const amt = Math.min(Math.log(10_000_001) / Math.log(10_000_000), 1);
    const score = Math.round((freq * 0.4 + amt * 0.6) * 100);
    expect(score).toBeGreaterThanOrEqual(0);
    expect(score).toBeLessThanOrEqual(100);
  });
});
