use std::collections::HashMap;

use anyhow::{Context, Error};
use jupiter_amm_interface::{AccountMap, Amm, AmmContext, ClockRef, KeyedAccount, QuoteParams, SwapMode};

use solana_client::{
    nonblocking::rpc_client::{RpcClient},
    rpc_request::RpcRequest,
    rpc_response::{Response, RpcKeyedAccount, RpcResponseContext},
    rpc_sender::{RpcSender, RpcTransportStats},
};
use solana_program_test::{BanksClient, BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::Account, clock::Clock, compute_budget::ComputeBudgetInstruction,
    instruction::Instruction, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    signature::Keypair, signer::Signer, sysvar, transaction::Transaction,
};
use woofi_jupiter::{util::SOL, util::USDC, WoofiSwap};

#[tokio::test]
// TODO replace with local accounts
async fn test_jupiter_quote() -> Result<(), Error> {

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let account = client.get_account(&woofi_jupiter::id()).await?;

    let amm_context = get_amm_context(&client).await?;

    let market_account = KeyedAccount {
        key: woofi_jupiter::id(),
        account,
        params: None,
    };

    let mut woofi_swap = WoofiSwap::from_keyed_account(&market_account, &amm_context).unwrap();
    println!("woofi_swap.token_a_mint:{}", woofi_swap.token_a_mint);
    println!("woofi_swap.token_a_wooracle:{}", woofi_swap.token_a_wooracle);
    println!("woofi_swap.token_a_woopool:{}", woofi_swap.token_a_woopool);
    println!("woofi_swap.token_a_feed_account:{}", woofi_swap.token_a_feed_account);
    println!("woofi_swap.token_a_price_update:{}", woofi_swap.token_a_price_update);

    println!("woofi_swap.token_b_mint:{}", woofi_swap.token_b_mint);
    println!("woofi_swap.token_b_wooracle:{}", woofi_swap.token_b_wooracle);
    println!("woofi_swap.token_b_woopool:{}", woofi_swap.token_b_woopool);
    println!("woofi_swap.token_b_feed_account:{}", woofi_swap.token_b_feed_account);
    println!("woofi_swap.token_b_price_update:{}", woofi_swap.token_b_price_update);

    let pubkeys = woofi_swap.get_accounts_to_update();
    let accounts_map: AccountMap = pubkeys
        .iter()
        .zip(client.get_multiple_accounts(&pubkeys).await?)
        .map(|(key, acc)| (*key, acc.unwrap()))
        .collect();

    woofi_swap.update(&accounts_map)?;
    let result = woofi_swap.quote(&QuoteParams{
        amount: 1000,
        input_mint: SOL,
        output_mint: USDC,
        swap_mode: SwapMode::ExactIn
    })?;

    println!("result.out_amount:{}", result.out_amount);
    println!("result.in_amount:{}", result.in_amount);
    println!("result.fee_amount:{}", result.fee_amount);
    println!("result.fee_mint:{}", result.fee_mint);

    Ok(())
}

pub async fn get_clock(rpc_client: &RpcClient) -> anyhow::Result<Clock> {
    let clock_data = rpc_client
        .get_account_with_commitment(&sysvar::clock::ID, rpc_client.commitment()).await?
        .value
        .context("Failed to get clock account")?;

    let clock: Clock = bincode::deserialize(&clock_data.data)
        .context("Failed to deserialize sysvar::clock::ID")?;

    Ok(clock)
}

pub async fn get_clock_ref(rpc_client: &RpcClient) -> anyhow::Result<ClockRef> {
    let clock = get_clock(rpc_client).await?;
    Ok(ClockRef::from(clock))
}

pub async fn get_amm_context(rpc_client: &RpcClient) -> anyhow::Result<AmmContext> {
    Ok(AmmContext {
        clock_ref: get_clock_ref(rpc_client).await?,
    })
}
