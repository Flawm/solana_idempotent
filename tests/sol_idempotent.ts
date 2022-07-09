import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolIdempotent } from '../target/types/sol_idempotent';

describe('sol_idempotent', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolIdempotent as Program<SolIdempotent>,
    payer = program.provider.wallet,
    systemProgram = new anchor.web3.PublicKey(
      '11111111111111111111111111111111',
    ),
    new_account = anchor.web3.Keypair.generate(),
    ctx = {
      accounts: {
        payer: payer.publicKey,
        map: new_account.publicKey,
      },
    };

  it('creates maps', async () => {
    const tx = new anchor.web3.Transaction(),
      local_ctx = {
        accounts: {
          ...ctx.accounts,
          systemProgram,
        },
      };

    const signers = [payer.payer, new_account];

    tx.feePayer = payer.publicKey;
    tx.add(await program.instruction.createMap(10, local_ctx));
    let a = await program.provider.sendAndConfirm(tx, signers, {
      skipPreflight: true,
      commitment: 'confirmed',
    });
    console.log(a);
  });

  it('marks bits', async () => {
    const tx = new anchor.web3.Transaction(),
      signers = [payer.payer];

    tx.feePayer = payer.publicKey;

    tx.add(await program.instruction.markBit(0, ctx));
    let a = await program.provider.sendAndConfirm(tx, signers, {
      skipPreflight: true,
      commitment: 'confirmed',
    });
    console.log(a);
  });

  it('fails when a bit is already marked', async () => {
    const tx = new anchor.web3.Transaction(),
      signers = [payer.payer];

    tx.feePayer = payer.publicKey;

    let pass = false;
    try {
      tx.add(await program.instruction.markBit(0, ctx));
      let a = await program.provider.sendAndConfirm(tx, signers, {
        skipPreflight: true,
        commitment: 'confirmed',
      });
      console.log(a);
    } catch (e: any) {
      pass = true;
    }

    if (!pass) {
      throw 'Failure';
    }
  });
});
