use anyhow::{Context, Error};
use jupiter_amm_interface::{
    AccountMap, Amm, AmmContext, ClockRef, KeyedAccount, QuoteParams, SwapMode,
};

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{clock::Clock, sysvar};
use woofi_jupiter::{util::{get_wooammpool_address, get_wooconfig_address, SOL, USDC}, WoofiSwap};

#[tokio::test]
// TODO replace with local accounts
async fn test_jupiter_quote() -> Result<(), Error> {
    // devnet
    //let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    let program_id = woofi_jupiter::id();
    
    let token_mint_a = SOL;
    let token_mint_b = USDC;

    let wooconfig = get_wooconfig_address(&program_id).0;
    let wooammpool = get_wooammpool_address(
                              &wooconfig,
                              &token_mint_a,
                              &token_mint_b,
                              &program_id
                          ).0;

    let account = client.get_account(&wooammpool).await?;
    let market_account = KeyedAccount {
        key: wooammpool,
        account,
        params: None,
    };

    let amm_context = get_amm_context(&client).await?;

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
