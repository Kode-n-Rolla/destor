import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { Destor } from "../target/types/destor";
import { expect } from "chai";
import { testAdmin, airdrop } from "./helpers/actors";
import { ensureProtocolInitialized } from "./helpers/protocol";

describe("destor::protocol", () => {
  ///////////////////////////////
  //       Preparation         //
  ///////////////////////////////
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.destor as Program<Destor>;

  const airdropSolAmount = 5;

  // Actors
  //let admin: anchor.web3.Keypair;
  //let bob: anchor.web3.Keypair;
  //let charlie: anchor.web3.Keypair;

  ///////////////////////////////
  //    Foundry-like setUp     //
  ///////////////////////////////
  before(async () => {
    await airdrop(provider, testAdmin.publicKey, airdropSolAmount);
  });

  ///////////////////////////////
  //     Happy Path Tests      //
  ///////////////////////////////
  it("create a protocol", async () => {
    const protocolAccount = await ensureProtocolInitialized(program, testAdmin);
    const accountInfo = await provider.connection.getAccountInfo(
      protocolAccount.protocolPda
    );

    // Assertions
    expect(accountInfo).to.be.not.null;
    expect(
      protocolAccount.protocolConfig.admin.equals(testAdmin.publicKey)
    ).to.eq(true);
    expect(protocolAccount.protocolConfig.bump).to.eq(
      protocolAccount.protocolBump
    );
  });
});
