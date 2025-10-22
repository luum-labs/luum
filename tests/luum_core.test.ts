import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID, TIER_THRESHOLDS_BPS } from "../sdk/src/constants";

describe("luum_core", () => {
  test("program ID is valid base58", () => {
    const pk = new PublicKey(PROGRAM_ID);
    expect(pk.toBase58()).toBe(PROGRAM_ID);
  });

  test("tier thresholds are in ascending order", () => {
    for (let i = 1; i < TIER_THRESHOLDS_BPS.length; i++) {
      expect(TIER_THRESHOLDS_BPS[i]).toBeGreaterThan(TIER_THRESHOLDS_BPS[i - 1]);
    }
  });

  test("analysis seed derivation is deterministic", () => {
    const wallet = PublicKey.default;
    const authority = PublicKey.default;
    const [pda1] = PublicKey.findProgramAddressSync(
      [Buffer.from("analysis"), authority.toBuffer(), wallet.toBuffer()],
      new PublicKey(PROGRAM_ID)
    );
    const [pda2] = PublicKey.findProgramAddressSync(
      [Buffer.from("analysis"), authority.toBuffer(), wallet.toBuffer()],
      new PublicKey(PROGRAM_ID)
    );
    expect(pda1.toBase58()).toBe(pda2.toBase58());
  });
});
