use std::collections::HashMap;

use anyhow::{Context, Error};
use jupiter_amm_interface::{Amm, AmmContext, ClockRef, KeyedAccount, SwapMode};

use solana_client::{
    nonblocking::rpc_client::{RpcClient},
    rpc_request::RpcRequest,
    rpc_response::{Response, RpcKeyedAccount, RpcResponseContext},
    rpc_sender::{RpcSender, RpcTransportStats},
};
use solana_program_test::{BanksClient, BanksClientError, ProgramTestContext};
use solana_sdk::{
    account::Account, clock::Clock, compute_budget::ComputeBudgetInstruction,
    instruction::Instruction, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    signature::Keypair, signer::Signer, sysvar, transaction::Transaction,
};
use woofi_jupiter::WoofiSwap;
use woofi_jupiter::util::WOOFI_PROGRAM_ID;

#[tokio::test]
// TODO replace with local accounts
async fn test_jupiter_local() -> Result<(), Error> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let account = client.get_account(&WOOFI_PROGRAM_ID).await?;

    let amm_context = get_amm_context(&client).await?;

    let market_account = KeyedAccount {
        key: WOOFI_PROGRAM_ID,
        account,
        params: None,
    };

    let woofi_swap = WoofiSwap::from_keyed_account(&market_account, &amm_context).unwrap();
    //println!("woofi {} {} {} {}",
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


