import * as anchor from "@anchor-lang/core";

export const testAdmin = anchor.web3.Keypair.generate();

export const airdrop = async (
  provider: anchor.AnchorProvider,
  pubkey: anchor.web3.PublicKey,
  sol: number
) => {
  const sig = await provider.connection.requestAirdrop(
    pubkey,
    sol * anchor.web3.LAMPORTS_PER_SOL
  );

  const latest = await provider.connection.getLatestBlockhash();

  await provider.connection.confirmTransaction(
    {
      signature: sig,
      blockhash: latest.blockhash,
      lastValidBlockHeight: latest.lastValidBlockHeight,
    },
    "confirmed"
  );
};
