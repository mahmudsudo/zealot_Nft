import * as anchor from '@coral-xyz/anchor'
import { TOKEN_PROGRAM_ID, createInitializeMintInstruction, getMintLen } from '@solana/spl-token'
import { Keypair, PublicKey, SystemProgram } from '@solana/web3.js'
import { assert } from 'chai'

import { OftTools } from '@layerzerolabs/lz-solana-sdk-v2'

import onftIdl from '../target/idl/zealot_nft.json'
import endpointIdl from '../target/idl/endpoint.json'

const ONFT_SEED = 'ONft'
const SOLANA_ONFT_TOKEN_DECIMALS = 8
const ONFT_SHARE_DECIMALS = 6

describe('onft', () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local(undefined, {
        commitment: 'confirmed',
        preflightCommitment: 'confirmed',
    })
    const wallet = provider.wallet as anchor.Wallet
    const ONFT_PROGRAM_ID = new PublicKey(onftIdl.metadata.address)
    const ENDPOINT_PROGRAM_ID = new PublicKey(endpointIdl.metadata.address)
    const mintKp = Keypair.generate()
        const [onftConfigPda] = PublicKey.findProgramAddressSync(
            [Buffer.from(ONFT_SEED, 'utf8'), mintKp.publicKey.toBuffer()],
            new anchor.web3.PublicKey(onftIdl.metadata.address)
        )

    it('Initialize ONFT', async () => {
        
        // step 1, create the mint token
        const createMintIxs = [
            SystemProgram.createAccount({
                fromPubkey: wallet.publicKey,
                newAccountPubkey: mintKp.publicKey,
                space: getMintLen([]),
                lamports: await provider.connection.getMinimumBalanceForRentExemption(getMintLen([])),
                programId: TOKEN_PROGRAM_ID,
            }),
            createInitializeMintInstruction(mintKp.publicKey, SOLANA_ONFT_TOKEN_DECIMALS, onftConfigPda, onftConfigPda),
        ]
        await provider.sendAndConfirm(new anchor.web3.Transaction().add(...createMintIxs), [wallet.payer, mintKp])

        // step 2, create the OFT token
        const initOftIx = await OftTools.createInitNativeOftIx(
            wallet.publicKey,
            wallet.publicKey,
            mintKp.publicKey,
            wallet.publicKey,
            ONFT_SHARE_DECIMALS,
            TOKEN_PROGRAM_ID,
            ONFT_PROGRAM_ID,
            ENDPOINT_PROGRAM_ID
        )

        await provider.sendAndConfirm(new anchor.web3.Transaction().add(initOftIx), [wallet.payer])

        // check status
        const delegate = await OftTools.getDelegate(provider.connection, onftConfigPda, ENDPOINT_PROGRAM_ID)
        assert.equal(delegate.toBase58(), wallet.publicKey.toBase58())
    }),
    it('Test Send', async () => {
       
        // step 1, create the mint token
        const createMintIxs = [
            SystemProgram.createAccount({
                fromPubkey: wallet.publicKey,
                newAccountPubkey: mintKp.publicKey,
                space: getMintLen([]),
                lamports: await provider.connection.getMinimumBalanceForRentExemption(getMintLen([])),
                programId: TOKEN_PROGRAM_ID,
            }),
            createInitializeMintInstruction(mintKp.publicKey, SOLANA_ONFT_TOKEN_DECIMALS, onftConfigPda, onftConfigPda),
        ]
        await provider.sendAndConfirm(new anchor.web3.Transaction().add(...createMintIxs), [wallet.payer, mintKp])

        // step 2, create the OFT token
        const initOnftIx = await OftTools.createInitNativeOftIx(
            wallet.publicKey,
            wallet.publicKey,
            mintKp.publicKey,
            wallet.publicKey,
            ONFT_SHARE_DECIMALS,
            TOKEN_PROGRAM_ID,
            ONFT_PROGRAM_ID,
            ENDPOINT_PROGRAM_ID
        )
        await provider.sendAndConfirm(new anchor.web3.Transaction().add(initOnftIx), [wallet.payer])

        // check status
        const delegate = await OftTools.getDelegate(provider.connection, onftConfigPda, ENDPOINT_PROGRAM_ID)
        assert.equal(delegate.toBase58(), wallet.publicKey.toBase58())
    }),
    it('Test receive', async () => {
       
        // step 1, create the mint token
        const createMintIxs = [
            SystemProgram.createAccount({
                fromPubkey: wallet.publicKey,
                newAccountPubkey: mintKp.publicKey,
                space: getMintLen([]),
                lamports: await provider.connection.getMinimumBalanceForRentExemption(getMintLen([])),
                programId: TOKEN_PROGRAM_ID,
            }),
            createInitializeMintInstruction(mintKp.publicKey, SOLANA_ONFT_TOKEN_DECIMALS, onftConfigPda, onftConfigPda),
        ]
        await provider.sendAndConfirm(new anchor.web3.Transaction().add(...createMintIxs), [wallet.payer, mintKp])

        // step 2, create the OFT token
        const initOnftIx = await OftTools.createInitNativeOftIx(
            wallet.publicKey,
            wallet.publicKey,
            mintKp.publicKey,
            wallet.publicKey,
            ONFT_SHARE_DECIMALS,
            TOKEN_PROGRAM_ID,
            ONFT_PROGRAM_ID,
            ENDPOINT_PROGRAM_ID
        )
       
        await provider.sendAndConfirm(new anchor.web3.Transaction().add(initOnftIx), [wallet.payer])

        // check status
        const delegate = await OftTools.getDelegate(provider.connection, onftConfigPda, ENDPOINT_PROGRAM_ID)
        assert.equal(delegate.toBase58(), wallet.publicKey.toBase58())
    }),
})
