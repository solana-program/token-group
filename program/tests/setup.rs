use {
    solana_program_test::{processor, tokio::sync::Mutex, ProgramTest, ProgramTestContext},
    solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    spl_token_client::{
        client::{
            ProgramBanksClient, ProgramBanksClientProcessTransaction, ProgramClient,
            SendTransaction, SimulateTransaction,
        },
        token::Token,
    },
    std::sync::Arc,
};

/// Set up a program test
pub async fn setup_program_test(
    program_id: &Pubkey,
) -> (
    Arc<Mutex<ProgramTestContext>>,
    Arc<dyn ProgramClient<ProgramBanksClientProcessTransaction>>,
    Arc<Keypair>,
) {
    let mut program_test = ProgramTest::new(
        "spl_token_group_example",
        *program_id,
        processor!(spl_token_group_example::processor::process),
    );
    // Token-2022 v12 cannot run as a native builtin, so always load it from
    // the BPF fixture at tests/fixtures/spl_token_2022.so
    program_test.prefer_bpf(true);
    program_test.add_program("spl_token_2022", spl_token_2022::id(), None);
    let context = program_test.start_with_context().await;
    let payer = Arc::new(context.payer.insecure_clone());
    let context = Arc::new(Mutex::new(context));
    let client: Arc<dyn ProgramClient<ProgramBanksClientProcessTransaction>> =
        Arc::new(ProgramBanksClient::new_from_context(
            Arc::clone(&context),
            ProgramBanksClientProcessTransaction,
        ));
    (context, client, payer)
}

/// Set up a Token-2022 mint
pub async fn setup_mint<T: SendTransaction + SimulateTransaction>(
    token_client: &Token<T>,
    mint_keypair: &Keypair,
    mint_authority_keypair: &Keypair,
) {
    token_client
        .create_mint(
            &mint_authority_keypair.pubkey(),
            None,
            vec![],
            &[mint_keypair],
        )
        .await
        .unwrap();
}
