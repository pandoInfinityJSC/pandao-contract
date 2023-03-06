import { AnchorProvider, BN, Program, setProvider, web3, workspace } from "@project-serum/anchor"
import {
	createAssociatedTokenAccountInstruction,
	getAssociatedTokenAddress,
	getAssociatedTokenAddressSync,
} from "@solana/spl-token"
import {
	wallet,
	associatedTokenProgram,
	hackathonProgram,
	systemProgram,
	time,
	tokenProgram,
} from "./helpers/constants"
import {
	sendAndConfirmTransaction,
	claimTransaction,
	createProposal,
	grantAdminTransaction,
	initializeTransaction,
	revokeAdminTransaction,
	voteTransaction,
} from "./helpers/common"

interface Proposal {
	authority: web3.PublicKey
	dao: web3.PublicKey
	title: string
	description: string
	tokenVote: web3.PublicKey
	startTime: BN
	endTime: BN
	voteType: number
	numOfOptions: number
	numOfNftPerOption: number[]
	tokensPerOption: number
	threshold: number
	maxOptionsPerVote: number
}

describe("hackathon", () => {
	it("Get all proposals", async () => {
		const proposals = await hackathonProgram.account.proposal.all()
		console.log(proposals[0].account, proposals[0].account)
		return proposals
	})

	it("Get all proposals which user can vote", async () => {
		const allProposals = await hackathonProgram.account.proposal.all()
		const proposals = allProposals.filter((proposal) => {
			const { tokenVote } = <Proposal>proposal.account
			let myTokenAccount: web3.PublicKey

			try {
				getAssociatedTokenAddressSync(tokenVote, wallet.publicKey)
			} catch (error) {
				return false
			}

			return 
		})
	})
})
