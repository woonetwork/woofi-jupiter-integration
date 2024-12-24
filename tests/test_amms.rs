use anchor_lang::AccountDeserialize;
use anyhow::{Context, Error};
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, ClockRef, KeyedAccount, QuoteParams, SwapMode,
};

use serde_json::json;
use solana_client::{ nonblocking::rpc_client::RpcClient};
use solana_sdk::{clock::Clock, pubkey::Pubkey, sysvar};
use woofi_jupiter::{state::{WooPool, Wooracle}, util::{get_wooconfig_address, get_woopool_address, SOL, USDC}, WoofiSwap};

#[tokio::test]
// TODO replace with local accounts
async fn test_jupiter_quote() -> Result<(), Error> {
    // devnet
    //let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let program_id = woofi_jupiter::id();
    
    let quote_mint = USDC;
    let token_a_mint = SOL;
    let token_b_mint = USDC;

    let wooconfig = get_wooconfig_address(&program_id).0;
    let token_a_woopool_pubkey = get_woopool_address(&wooconfig, &token_a_mint, &quote_mint, &program_id).0;
    let (token_a_wooracle_pubkey, token_a_wooracle) = get_wooracle(token_a_woopool_pubkey, &client).await?;
    
    let token_b_woopool_pubkey = get_woopool_address(&wooconfig, &token_b_mint, &quote_mint, &program_id).0;
    let (token_b_wooracle_pubkey, token_b_wooracle) = get_wooracle(token_b_woopool_pubkey, &client).await?;

    let quote_pool_pubkey = get_woopool_address(&wooconfig, &quote_mint, &quote_mint, &program_id).0;
    let (_, quote_wooracle) = get_wooracle(quote_pool_pubkey, &client).await?;

    let keyed_account_params = json!({ 
        "token_a_wooracle": token_a_wooracle_pubkey.to_string(),
        "token_a_woopool": token_a_woopool_pubkey.to_string(),
        "token_a_feed_account": token_a_wooracle.feed_account.to_string(),
        "token_a_price_update": token_a_wooracle.price_update.to_string(),
        "token_b_wooracle": token_b_wooracle_pubkey.to_string(),
        "token_b_woopool": token_b_woopool_pubkey.to_string(),
        "token_b_feed_account": token_b_wooracle.feed_account.to_string(),
        "token_b_price_update": token_b_wooracle.price_update.to_string(),
        "quote_pool": quote_pool_pubkey.to_string(),
        "quote_price_update": quote_wooracle.price_update.to_string(),
    });

    let account = client.get_account(&program_id).await?;

    let amm_context = get_amm_context(&client).await?;

    let market_account = KeyedAccount {
        key: Pubkey::find_program_address(&[b"woofi_jupiter", 
                    token_a_mint.as_ref(), token_b_mint.as_ref(),
                    quote_mint.as_ref()], &program_id).0,
        account,
        params: Some(keyed_account_params),
    };

    let mut woofi_swap = WoofiSwap::from_keyed_account(&market_account, &amm_context).unwrap();

    let pubkeys = woofi_swap.get_accounts_to_update();
    let accounts_map: AccountMap = pubkeys
        .iter()
        .zip(client.get_multiple_accounts(&pubkeys).await?)
        .map(|(key, acc)| (*key, acc.unwrap()))
        .collect();

    woofi_swap.update(&accounts_map)?;

    let mut result = woofi_swap.quote(&QuoteParams {
        amount: 10000000,
        input_mint: SOL,
        output_mint: USDC,
        swap_mode: SwapMode::ExactIn,
    })?;

    println!("Getting quote for selling 0.01 SOL");
    println!("result.out_amount:{}", result.out_amount);
    println!("result.in_amount:{}", result.in_amount);
    println!("result.fee_amount:{}", result.fee_amount);
    println!("result.fee_mint:{}", result.fee_mint);

    result = woofi_swap.quote(&QuoteParams {
        amount: 200000000,
        input_mint: USDC,
        output_mint: SOL,
        swap_mode: SwapMode::ExactIn,
    })?;

    println!("Getting quote for buying SOL using 200 USDC");
    println!("result.out_amount:{}", result.out_amount);
    println!("result.in_amount:{}", result.in_amount);
    println!("result.fee_amount:{}", result.fee_amount);
    println!("result.fee_mint:{}", result.fee_mint);

    Ok(())
}

pub async fn get_wooracle(woopool_pubkey: Pubkey, client: &RpcClient) -> anyhow::Result<(Pubkey, Wooracle)> {
    let woopool_data = client.get_account(&woopool_pubkey).await?;
    let woopool = & WooPool::try_deserialize(&mut woopool_data.data.as_slice())?;
    let wooracle_pubkey = woopool.wooracle;
    let wooracle_data = client.get_account(&wooracle_pubkey).await?;
    let wooracle = Wooracle::try_deserialize(&mut wooracle_data.data.as_slice())?;

    Ok((wooracle_pubkey, wooracle))
}

pub async fn get_clock(rpc_client: &RpcClient) -> anyhow::Result<Clock> {
    let clock_data = rpc_client
        .get_account_with_commitment(&sysvar::clock::ID, rpc_client.commitment())
        .await?
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
