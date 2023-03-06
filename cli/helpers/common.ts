import { web3, utils, Program, BN } from "@project-serum/anchor"
import {
	createInitializeMintInstruction,
	createMintToInstruction,
	getAssociatedTokenAddressSync,
	getMinimumBalanceForRentExemptAccount,
	MINT_SIZE,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token"
import { Hackathon } from "../idl/hackathon"
import { systemProgram } from "./constants"

export async function initializeTransaction(
	hackathonProgram: Program<Hackathon>,
	walletPublicKey: web3.PublicKey,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	const dao = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("dao"), walletPublicKey.toBuffer()],
		hackathonProgram.programId
	)[0]

	transaction.add(
		await hackathonProgram.methods
			.initialize()
			.accounts({
				authority: walletPublicKey,
				dao,
				systemProgram,
			})
			.instruction()
	)

	return transaction
}

export async function grantAdminTransaction(
	hackathonProgram: Program<Hackathon>,
	walletPublicKey: web3.PublicKey,
	admin: web3.PublicKey,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	const dao = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("dao"), walletPublicKey.toBuffer()],
		hackathonProgram.programId
	)[0]

	transaction.add(
		await hackathonProgram.methods
			.grantAdmin()
			.accounts({
				authority: walletPublicKey,
				dao,
				account: admin,
			})
			.instruction()
	)

	return transaction
}

export async function revokeAdminTransaction(
	hackathonProgram: Program<Hackathon>,
	walletPublicKey: web3.PublicKey,
	admin: web3.PublicKey,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	const dao = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("dao"), walletPublicKey.toBuffer()],
		hackathonProgram.programId
	)[0]

	transaction.add(
		await hackathonProgram.methods
			.revokeAdmin()
			.accounts({
				authority: walletPublicKey,
				dao,
				account: admin,
			})
			.instruction()
	)

	return transaction
}

export async function createProposal(
	hackathonProgram: Program<Hackathon>,
	superAdmin: web3.PublicKey,
	walletPublicKey: web3.PublicKey,
	tokenVote: web3.PublicKey,
	title: string,
	description: string,
	startTime: BN,
	endTime: BN,
	voteType: number,
	numOfOptions: number,
	tokensPerOption: number,
	threshold: number,
	maxOptionsPerVote: number,
	systemProgram: web3.PublicKey,
	tokenProgram: web3.PublicKey,
	time: web3.PublicKey,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	const dao = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("dao"), superAdmin.toBuffer()],
		hackathonProgram.programId
	)[0]

	const proposal = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("proposal"), walletPublicKey.toBuffer(), Buffer.from("0")],
		hackathonProgram.programId
	)[0]

	transaction.add(
		await hackathonProgram.methods
			.createProposal(
				title,
				description,
				startTime,
				endTime,
				voteType,
				numOfOptions,
				tokensPerOption,
				threshold,
				maxOptionsPerVote
			)
			.accounts({
				authority: walletPublicKey,
				dao,
				proposal,
				tokenVote,
				systemProgram,
				tokenProgram,
				time,
			})
			.instruction()
	)

	return transaction
}

export async function voteTransaction(
	hackathonProgram: Program<Hackathon>,
	superAdmin: web3.PublicKey,
	walletPublicKey: web3.PublicKey,
	proposal: web3.PublicKey,
	options: number[],
	tokenVote: web3.PublicKey,
	systemProgram: web3.PublicKey,
	tokenProgram: web3.PublicKey,
	associatedTokenProgram: web3.PublicKey,
	time: web3.PublicKey,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	const dao = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("dao"), superAdmin.toBuffer()],
		hackathonProgram.programId
	)[0]
	const voter = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("voter"), walletPublicKey.toBuffer(), proposal.toBuffer()],
		hackathonProgram.programId
	)[0]
	const authorityTokenAccount = getAssociatedTokenAddressSync(tokenVote, walletPublicKey)
	const proposalTokenAccount = getAssociatedTokenAddressSync(tokenVote, proposal)

	transaction.add(
		await hackathonProgram.methods
			.vote(options.join(","))
			.accounts({
				authority: walletPublicKey,
				superAdmin,
				dao,
				proposal,
				voter,
				tokenVote,
				authorityTokenAccount,
				proposalTokenAccount,
				systemProgram,
				tokenProgram,
				associatedTokenProgram,
				time,
			})
			.instruction()
	)

	return transaction
}

export async function claimTransaction(
	hackathonProgram: Program<Hackathon>,
	superAdmin: web3.PublicKey,
	walletPublicKey: web3.PublicKey,
	proposal: web3.PublicKey,
	tokenVote: web3.PublicKey,
	systemProgram: web3.PublicKey,
	tokenProgram: web3.PublicKey,
	associatedTokenProgram: web3.PublicKey,
	time: web3.PublicKey,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	const dao = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("dao"), superAdmin.toBuffer()],
		hackathonProgram.programId
	)[0]
	const voter = web3.PublicKey.findProgramAddressSync(
		[Buffer.from("voter"), walletPublicKey.toBuffer(), proposal.toBuffer()],
		hackathonProgram.programId
	)[0]
	const authorityTokenAccount = getAssociatedTokenAddressSync(tokenVote, walletPublicKey)
	const proposalTokenAccount = getAssociatedTokenAddressSync(tokenVote, proposal)

	transaction.add(
		await hackathonProgram.methods
			.claim()
			.accounts({
				authority: walletPublicKey,
				superAdmin,
				dao,
				proposal,
				voter,
				tokenVote,
				authorityTokenAccount,
				proposalTokenAccount,
				systemProgram,
				tokenProgram,
				associatedTokenProgram,
				time,
			})
			.instruction()
	)

	return transaction
}

export async function sendAndConfirmTransaction(
	connection: web3.Connection,
	signers: web3.Signer[],
	transactions: (web3.Transaction | web3.TransactionInstruction)[],
	transaction: web3.Transaction = new web3.Transaction()
): Promise<string> {
	transaction.add(...transactions)
	return web3.sendAndConfirmTransaction(connection, transaction, signers)
}

export const sleep = (s: number) => {
	return new Promise((resolve) => setTimeout(resolve, s * 1000))
}
