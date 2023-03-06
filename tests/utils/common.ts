import { web3, utils } from "@project-serum/anchor"
import {
	createInitializeMintInstruction,
	createMintToInstruction,
	getMinimumBalanceForRentExemptAccount,
	MINT_SIZE,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token"

export const systemProgram: web3.PublicKey = web3.SystemProgram.programId
export const tokenProgram: web3.PublicKey = utils.token.TOKEN_PROGRAM_ID
export const associatedTokenProgram: web3.PublicKey = utils.token.ASSOCIATED_PROGRAM_ID
export const rent: web3.PublicKey = web3.SYSVAR_RENT_PUBKEY
export const time: web3.PublicKey = web3.SYSVAR_CLOCK_PUBKEY
export const instruction: web3.PublicKey = web3.SYSVAR_INSTRUCTIONS_PUBKEY

export async function airdrop(connection: web3.Connection, user: web3.PublicKey): Promise<String> {
	return connection.requestAirdrop(user, 10 ** 9)
}

export async function createMintTransaction(
	connection: web3.Connection,
	walletPublicKey: web3.PublicKey,
	mint: web3.PublicKey,
	decimals: number = 0,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	let lamports = await getMinimumBalanceForRentExemptAccount(connection)
	transaction.add(
		web3.SystemProgram.createAccount({
			fromPubkey: walletPublicKey,
			newAccountPubkey: mint,
			space: MINT_SIZE,
			programId: TOKEN_PROGRAM_ID,
			lamports,
		}),
		createInitializeMintInstruction(mint, decimals, walletPublicKey, walletPublicKey)
	)
	return transaction
}

export async function mintToTransaction(
	mint: web3.PublicKey,
	walletPublicKey: web3.PublicKey,
	destination: web3.PublicKey,
	amount: number,
	transaction: web3.Transaction = new web3.Transaction()
): Promise<web3.Transaction> {
	transaction.add(createMintToInstruction(mint, destination, walletPublicKey, amount, [], TOKEN_PROGRAM_ID))
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
