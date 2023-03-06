import { web3, AnchorProvider, Wallet, Program, utils } from "@project-serum/anchor"
// const { decode } = require("bs58");

export const systemProgram: web3.PublicKey = web3.SystemProgram.programId
export const tokenProgram: web3.PublicKey = utils.token.TOKEN_PROGRAM_ID
export const associatedTokenProgram: web3.PublicKey = utils.token.ASSOCIATED_PROGRAM_ID
export const rent: web3.PublicKey = web3.SYSVAR_RENT_PUBKEY
export const time: web3.PublicKey = web3.SYSVAR_CLOCK_PUBKEY
export const instruction: web3.PublicKey = web3.SYSVAR_INSTRUCTIONS_PUBKEY

const opts: { preflightCommitment: web3.Commitment } = {
	preflightCommitment: "confirmed",
}
// const wallet = web3.Keypair.fromSecretKey(decode(process.env.WALLET_PRIVATE_KEY));
export const wallet = web3.Keypair.fromSecretKey(Uint8Array.from(require("../../wallet/deployer.json")))
export const connection = new web3.Connection("https://api.devnet.solana.com", opts.preflightCommitment)
export const provider = new AnchorProvider(connection, new Wallet(wallet), opts)

export const hackathonIdl = require("../idl/hackathon.json")

export const hackathonProgram = new Program(hackathonIdl, new web3.PublicKey(hackathonIdl.metadata.address), provider)

export const superAdmin = new web3.PublicKey("57p59zh19AWLXFnyFHcQAQnKtHWhuU9CRRmq1QC2WzUi")

export const tokenVote = new web3.PublicKey("4LbqZVewzmXD1oAicj8BuwW8hrTsZf135bZAzpxXpfUA")
