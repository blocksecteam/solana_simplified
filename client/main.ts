const fs = require("fs");
const path = require('path');
const web3 = require("@solana/web3.js");
import * as borsh from 'borsh';

const KEYPAIR_PATH = path.join(process.env.HOME, "/.config/solana/id.json");
const PROGRAM_ID = new web3.PublicKey("BPE4bWD9DjWDCjTewNbf9pDvDeRfsMDQu1tfk7rJpwL");
const POST_ARTICLE_SCHEMA = {struct: {'title': 'string', 'content': 'string'}};

function loadKeyFromFile(filePath: string) {
    const secretKeyString = fs.readFileSync(filePath, "utf8");
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return web3.Keypair.fromSecretKey(secretKey);
}

function bufferFromU8(u8: number): Buffer {
    const buffer = Buffer.alloc(1);
    buffer.writeUint8(u8, 0);
    return buffer;
}

function bufferFromU32(u32: number): Buffer {
    const buffer = Buffer.alloc(4);
    buffer.writeUint32LE(u32, 0);
    return buffer;
}

async function printTransactionLogs(connection, signature) {
    const response = await connection.getTransaction(signature, {commitment: 'confirmed'});
    console.log(`[*] logs for transaction ${signature}:`, response.meta?.logMessages);
}

const main = async () => {
    let payer = loadKeyFromFile(KEYPAIR_PATH);
    let indexPda = web3.PublicKey.findProgramAddressSync([Buffer.from('INDEX_PDA')], PROGRAM_ID)[0];
    // let connection = new web3.Connection("https://rpc.ankr.com/solana_devnet");
    let connection = new web3.Connection(web3.clusterApiUrl("devnet"));

    // init the program
    let init_tx = new web3.Transaction().add(
        new web3.TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: indexPda, isSigner: false, isWritable: true},
                {pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false},
            ],
            programId: PROGRAM_ID,
            data: bufferFromU8(0),
        }),
    );
    console.log("[*] send init transaction...");
    let init_tx_signature = await web3.sendAndConfirmTransaction(connection, init_tx, [payer]);
    console.log("[+] init transaction is confirmed: ", init_tx_signature);

    // post several articles
    const article1 = {title: "Hello", content: "Hello World!"};
    const article2 = {title: "Test title", content: "Test Content"};
    const enCodedArticle1 = borsh.serialize(POST_ARTICLE_SCHEMA, article1);
    const enCodedArticle2 = borsh.serialize(POST_ARTICLE_SCHEMA, article2);
    const instData1 = Buffer.concat([bufferFromU8(1), Buffer.from(enCodedArticle1)]);
    const instData2 = Buffer.concat([bufferFromU8(1), Buffer.from(enCodedArticle2)]);
    let articlePda1 = web3.PublicKey.findProgramAddressSync([Buffer.from('ARTICLE_PDA'), bufferFromU32(0)], PROGRAM_ID)[0];
    let articlePda2 = web3.PublicKey.findProgramAddressSync([Buffer.from('ARTICLE_PDA'), bufferFromU32(1)], PROGRAM_ID)[0];

    let post_tx = new web3.Transaction()
        .add(new web3.TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: indexPda, isSigner: false, isWritable: true},
                {pubkey: articlePda1, isSigner: false, isWritable: true},
                {pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false},
            ],
            programId: PROGRAM_ID,
            data: instData1,
        }))
        .add(new web3.TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: indexPda, isSigner: false, isWritable: true},
                {pubkey: articlePda2, isSigner: false, isWritable: true},
                {pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false},
            ],
            programId: PROGRAM_ID,
            data: instData2,
        }));
    console.log("[*] send article posting transaction...");
    let post_tx_signature = await web3.sendAndConfirmTransaction(connection, post_tx, [payer]);
    console.log("[+] article posting transaction is confirmed: ", post_tx_signature);

    // list articles
    let list_tx = new web3.Transaction().add(
        new web3.TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: indexPda, isSigner: false, isWritable: true},
                {pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false},
                {pubkey: articlePda1, isSigner: false, isWritable: false},
                {pubkey: articlePda2, isSigner: false, isWritable: false},
            ],
            programId: PROGRAM_ID,
            data: bufferFromU8(2),
        })
    );
    console.log("[*] send list article transaction...");
    const list_tx_signature = await web3.sendAndConfirmTransaction(connection, list_tx, [payer]);
    console.log("[+] list article transaction is confirmed: ", list_tx_signature);
    await printTransactionLogs(connection, list_tx_signature);
}

void main()
