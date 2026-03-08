import { appendTransactionMessageInstruction, generateKeyPairSigner, pipe } from '@trezoa/kit';
import test from 'ava';
import { Mint, Token, fetchMint, fetchToken, getMintToInstruction } from '../src';
import {
    createDefaultTrezoaClient,
    createDefaultTransaction,
    createMint,
    createToken,
    generateKeyPairSignerWithTrz,
    signAndSendTransaction,
} from './_setup';

test('it mints tokens to a token account', async t => {
    // Given a mint account and a token account.
    const client = createDefaultTrezoaClient();
    const [payer, mintAuthority, owner] = await Promise.all([
        generateKeyPairSignerWithTrz(client),
        generateKeyPairSigner(),
        generateKeyPairSigner(),
    ]);
    const mint = await createMint(client, payer, mintAuthority.address);
    const token = await createToken(client, payer, mint, owner.address);

    // When the mint authority mints tokens to the token account.
    const mintTo = getMintToInstruction({
        mint,
        token,
        mintAuthority,
        amount: 100n,
    });
    await pipe(
        await createDefaultTransaction(client, payer),
        tx => appendTransactionMessageInstruction(mintTo, tx),
        tx => signAndSendTransaction(client, tx),
    );

    // Then we expect the mint and token accounts to have the following updated data.
    const [{ data: mintData }, { data: tokenData }] = await Promise.all([
        fetchMint(client.rpc, mint),
        fetchToken(client.rpc, token),
    ]);
    t.like(mintData, <Mint>{ supply: 100n });
    t.like(tokenData, <Token>{ amount: 100n });
});
