import * as anchor from "@coral-xyz/anchor";

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