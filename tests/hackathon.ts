import { AnchorProvider, BN, Program, setProvider, web3, workspace } from "@project-serum/anchor"
import { createAssociatedTokenAccountInstruction, getAssociatedTokenAddress } from "@solana/spl-token"
import { Hackathon } from "../target/types/hackathon"
import {
	airdrop,
	associatedTokenProgram,
	createMintTransaction,
	mintToTransaction,
	sendAndConfirmTransaction,
	systemProgram,
	time,
	tokenProgram,
} from "./utils/common"
import {
	claimTransaction,
	createProposal,
	grantAdminTransaction,
	initializeTransaction,
	revokeAdminTransaction,
	voteTransaction,
} from "./utils/hackathon"

describe("hackathon", () => {
	let provider = AnchorProvider.env()
	setProvider(provider)

	let hackathonProgram = workspace.Hackathon as Program<Hackathon>
	let { connection } = hackathonProgram.provider
	let wallet: web3.Keypair = web3.Keypair.fromSecretKey(Uint8Array.from(require("../wallet/deployer.json")))
	let walletTokenAccount: web3.PublicKey
	let user: web3.Keypair = new web3.Keypair()
	let userTokenAccount: web3.PublicKey
	let tokenVote: web3.Keypair = web3.Keypair.generate()

	before(async () => {
		await airdrop(connection, user.publicKey)

		await sendAndConfirmTransaction(
			connection,
			[wallet, tokenVote],
			[await createMintTransaction(connection, wallet.publicKey, tokenVote.publicKey, 0)]
		)

		let transaction: web3.Transaction = new web3.Transaction()

		walletTokenAccount = await getAssociatedTokenAddress(tokenVote.publicKey, wallet.publicKey)
		if ((await connection.getAccountInfo(walletTokenAccount)) == null) {
			transaction.add(
				createAssociatedTokenAccountInstruction(
					wallet.publicKey,
					walletTokenAccount,
					wallet.publicKey,
					tokenVote.publicKey
				)
			)
		}
		transaction.add(await mintToTransaction(tokenVote.publicKey, wallet.publicKey, walletTokenAccount, 5000000))
		await sendAndConfirmTransaction(connection, [wallet], [transaction])

		transaction = new web3.Transaction()
		userTokenAccount = await getAssociatedTokenAddress(tokenVote.publicKey, user.publicKey)
		if ((await connection.getAccountInfo(userTokenAccount)) == null) {
			transaction.add(
				createAssociatedTokenAccountInstruction(
					user.publicKey,
					userTokenAccount,
					user.publicKey,
					tokenVote.publicKey
				)
			)
		}
		transaction.add(await mintToTransaction(tokenVote.publicKey, wallet.publicKey, userTokenAccount, 5000000))
		await sendAndConfirmTransaction(connection, [user, wallet], [transaction])
	})

	it("Initialize", async () => {
		await sendAndConfirmTransaction(
			connection,
			[wallet],
			[await initializeTransaction(hackathonProgram, wallet.publicKey)]
		)
	})

	it("Grant admin", async () => {
		await sendAndConfirmTransaction(
			connection,
			[wallet],
			[await grantAdminTransaction(hackathonProgram, wallet.publicKey, user.publicKey)]
		)
	})

	it("Revoke admin", async () => {
		await sendAndConfirmTransaction(
			connection,
			[wallet],
			[await revokeAdminTransaction(hackathonProgram, wallet.publicKey, user.publicKey)]
		)
	})

	it("Create proposal", async () => {
		const now = Math.floor(new Date().getTime() / 1000)
		const start = now + 10 * 60
		const end = start + 100
		await sendAndConfirmTransaction(
			connection,
			[wallet],
			[
				await createProposal(
					hackathonProgram,
					wallet.publicKey,
					wallet.publicKey,
					tokenVote.publicKey,
					"Proposal title",
					"Proposal description",
					new BN(start),
					new BN(end),
					0,
					5,
					2,
					500,
					1,
					systemProgram,
					tokenProgram,
					time
				),
			]
		)
	})

	it("Vote", async () => {
		const proposal = web3.PublicKey.findProgramAddressSync(
			[Buffer.from("proposal"), wallet.publicKey.toBuffer(), Buffer.from("0")],
			hackathonProgram.programId
		)[0]

		try {
			await sendAndConfirmTransaction(
				connection,
				[wallet],
				[
					await voteTransaction(
						hackathonProgram,
						wallet.publicKey,
						wallet.publicKey,
						proposal,
						[3],
						tokenVote.publicKey,
						systemProgram,
						tokenProgram,
						associatedTokenProgram,
						time
					),
				]
			)
		} catch (error) {
			console.log(error.logs);
		}
	})

	it("Claim", async () => {
		const proposal = web3.PublicKey.findProgramAddressSync(
			[Buffer.from("proposal"), wallet.publicKey.toBuffer(), Buffer.from("0")],
			hackathonProgram.programId
		)[0]

		try {
			await sendAndConfirmTransaction(
				connection,
				[wallet],
				[
					await claimTransaction(
						hackathonProgram,
						wallet.publicKey,
						wallet.publicKey,
						proposal,
						tokenVote.publicKey,
						systemProgram,
						tokenProgram,
						associatedTokenProgram,
						time
					),
				]
			)
		} catch (error) {
			console.log(error.logs);
		}
	})
})
