import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { BiDirecrtionalPayments } from '../target/types/bi_direcrtional_payments';

describe('bi-direcrtional_payments', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.BiDirecrtionalPayments as Program<BiDirecrtionalPayments>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
