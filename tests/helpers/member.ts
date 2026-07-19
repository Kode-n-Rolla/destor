import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Destor } from "../../target/types/destor";

type CreateMemberArgs = {
    program: Program<Destor>;
    organizationPda: anchor.web3.PublicKey;
    authority: anchor.web3.Keypair;
    wallet?: anchor.web3.Keypair;
};

export const getMemberPda = (
    programId: anchor.web3.PublicKey,
    organizationPda: anchor.web3.PublicKey,
    wallet: anchor.web3.PublicKey
) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("member"),
            organizationPda.toBuffer(),
            wallet.toBuffer()
        ],
        programId
    );
};

export const createMember = async ({
    program,
    organizationPda,
    authority,
    wallet = anchor.web3.Keypair.generate(),
}: CreateMemberArgs) => {
    const [memberPda, memberBump] = getMemberPda(
        program.programId,
        organizationPda,
        wallet.publicKey
    );

    await program.methods
        .addOrganizationMember(wallet.publicKey)
        .accountsPartial({
            authority: authority.publicKey,
            member: memberPda,
            organization: organizationPda
        })
        .signers([authority])
        .rpc();
    
    const memberAccount = await program.account.member.fetch(memberPda);

    return {
        wallet,
        memberPda,
        memberBump,
        memberAccount
    };
};