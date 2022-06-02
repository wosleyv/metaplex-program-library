#![cfg(feature = "test-bpf")]
pub mod common;
pub mod utils;

use anchor_lang::AccountDeserialize;
use common::*;
use utils::{helpers::default_scopes, setup_functions::*};

use mpl_auction_house::receipt::ListingReceipt;
use mpl_testing_utils::{solana::airdrop, utils::Metadata};
use solana_sdk::{program_pack::Pack, signer::Signer, sysvar::clock::Clock};
use spl_token::state::Account;
use std::assert_eq;

#[tokio::test]
async fn sell_success() {
    let mut context = auction_house_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    let owner_pubkey = &test_metadata.token.pubkey();
    airdrop(&mut context, owner_pubkey, TEN_SOL).await.unwrap();
    test_metadata
        .create(
            &mut context,
            "Test".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
        )
        .await
        .unwrap();
    let ((acc, listing_receipt_acc), sell_tx) =
        sell(&mut context, &ahkey, &ah, &test_metadata, 1, 1);

    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();
    let sts = context
        .banks_client
        .get_account(acc.seller_trade_state)
        .await
        .expect("Error Getting Trade State")
        .expect("Trade State Empty");
    assert_eq!(sts.data.len(), 1);

    let timestamp = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;

    let listing_receipt_account = context
        .banks_client
        .get_account(listing_receipt_acc.receipt)
        .await
        .expect("getting listing receipt")
        .expect("empty listing receipt data");

    let listing_receipt =
        ListingReceipt::try_deserialize(&mut listing_receipt_account.data.as_ref()).unwrap();

    assert_eq!(listing_receipt.auction_house, acc.auction_house);
    assert_eq!(listing_receipt.metadata, acc.metadata);
    assert_eq!(listing_receipt.seller, acc.wallet);
    assert_eq!(listing_receipt.created_at, timestamp);
    assert_eq!(listing_receipt.purchase_receipt, None);
    assert_eq!(listing_receipt.canceled_at, None);
    assert_eq!(listing_receipt.bookkeeper, *owner_pubkey);
    assert_eq!(listing_receipt.seller, *owner_pubkey);
    assert_eq!(listing_receipt.price, 1);
    assert_eq!(listing_receipt.token_size, 1);
}

#[tokio::test]
async fn auctioneer_sell_success() {
    let mut context = auction_house_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, ah_auth) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    let owner_pubkey = &test_metadata.token.pubkey();
    airdrop(&mut context, owner_pubkey, TEN_SOL).await.unwrap();
    test_metadata
        .create(
            &mut context,
            "Test".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
        )
        .await
        .unwrap();

    let sale_price = ONE_SOL;

    // Delegate external auctioneer authority.
    let auctioneer_authority = Keypair::new();
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority.pubkey());

    delegate_auctioneer(
        &mut context,
        ahkey,
        &ah_auth,
        auctioneer_authority.pubkey(),
        auctioneer_pda,
        default_scopes(),
    )
    .await
    .unwrap();

    let ((acc, listing_receipt_acc), sell_tx) = auctioneer_sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        sale_price,
        &auctioneer_authority.pubkey(),
    );

    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();

    let sts = context
        .banks_client
        .get_account(acc.seller_trade_state)
        .await
        .expect("Error Getting Trade State")
        .expect("Trade State Empty");
    assert_eq!(sts.data.len(), 1);

    let timestamp = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;

    let listing_receipt_account = context
        .banks_client
        .get_account(listing_receipt_acc.receipt)
        .await
        .expect("getting listing receipt")
        .expect("empty listing receipt data");

    let listing_receipt =
        ListingReceipt::try_deserialize(&mut listing_receipt_account.data.as_ref()).unwrap();

    assert_eq!(listing_receipt.auction_house, acc.auction_house);
    assert_eq!(listing_receipt.metadata, acc.metadata);
    assert_eq!(listing_receipt.seller, acc.wallet);
    assert_eq!(listing_receipt.created_at, timestamp);
    assert_eq!(listing_receipt.purchase_receipt, None);
    assert_eq!(listing_receipt.canceled_at, None);
    assert_eq!(listing_receipt.bookkeeper, *owner_pubkey);
    assert_eq!(listing_receipt.seller, *owner_pubkey);
    assert_eq!(listing_receipt.price, sale_price);
    assert_eq!(listing_receipt.token_size, 1);
}

#[tokio::test]
async fn auctioneer_sell_missing_scope_fails() {
    let mut context = auction_house_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, ah_auth) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    let owner_pubkey = &test_metadata.token.pubkey();
    airdrop(&mut context, owner_pubkey, TEN_SOL).await.unwrap();
    test_metadata
        .create(
            &mut context,
            "Test".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
        )
        .await
        .unwrap();

    let sale_price = ONE_SOL;

    // Delegate external auctioneer authority.
    let auctioneer_authority = Keypair::new();
    let (auctioneer_pda, _) = find_auctioneer_pda(&ahkey, &auctioneer_authority.pubkey());

    let scopes = vec![];

    delegate_auctioneer(
        &mut context,
        ahkey,
        &ah_auth,
        auctioneer_authority.pubkey(),
        auctioneer_pda,
        scopes.clone(),
    )
    .await
    .unwrap();

    let ((_, _), sell_tx) = auctioneer_sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        sale_price,
        &auctioneer_authority.pubkey(),
    );

    let error = context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap_err();
    assert_error!(error, MISSING_AUCTIONEER_SCOPE);
}

#[tokio::test]
async fn auctioneer_sell_no_delegate_fails() {
    let mut context = auction_house_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    let owner_pubkey = &test_metadata.token.pubkey();
    airdrop(&mut context, owner_pubkey, TEN_SOL).await.unwrap();
    test_metadata
        .create(
            &mut context,
            "Test".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            10,
            false,
        )
        .await
        .unwrap();

    let sale_price = ONE_SOL;

    // Delegate external auctioneer authority.
    let auctioneer_authority = Keypair::new();

    let ((_acc, _listing_receipt_acc), sell_tx) = auctioneer_sell(
        &mut context,
        &ahkey,
        &ah,
        &test_metadata,
        sale_price,
        &auctioneer_authority.pubkey(),
    );

    let error = context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap_err();

    assert_error!(error, INVALID_SEEDS);
}

#[tokio::test]
async fn sell_partial_order_success() {
    let mut context = auction_house_program_test().start_with_context().await;
    // Payer Wallet
    let (ah, ahkey, _) = existing_auction_house_test_context(&mut context)
        .await
        .unwrap();
    let test_metadata = Metadata::new();
    let owner_pubkey = &test_metadata.token.pubkey();
    airdrop(&mut context, owner_pubkey, TEN_SOL).await.unwrap();
    test_metadata
        .create(
            &mut context,
            "Test".to_string(),
            "TST".to_string(),
            "uri".to_string(),
            None,
            0,
            false,
        )
        .await
        .unwrap();

    // let md_account = get_account(&mut context, &test_nft.pubkey).await;
    // let metadata = ProgramMetadata::deserialize(&mut md_account.data.as_slice()).unwrap();

    // Check that token standard is not set.
    // assert_eq!(metadata.token_standard, None);

    // let ix = set_token_standard(
    //     PROGRAM_ID,
    //     test_nft.pubkey,
    //     context.payer.pubkey(),
    //     test_nft.mint.pubkey(),
    //     None,
    // );
    // let tx = Transaction::new_signed_with_payer(
    //     &[ix],
    //     Some(&context.payer.pubkey()),
    //     &[&context.payer],
    //     context.last_blockhash,
    // );
    // context.banks_client.process_transaction(tx).await.unwrap();

    let token_account = get_associated_token_address(&owner_pubkey, &test_metadata.mint.pubkey());
    let token_account_data = Account::unpack_from_slice(
        context
            .banks_client
            .get_account(token_account)
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    // Check that token standard has been updated successfully.
    // assert_eq!(metadata.token_standard, Some(TokenStandard::FungibleAsset));
    assert_eq!(token_account_data.amount, 6);

    let ((acc, listing_receipt_acc), sell_tx) =
        sell(&mut context, &ahkey, &ah, &test_metadata, 1, 6);

    context
        .banks_client
        .process_transaction(sell_tx)
        .await
        .unwrap();
    let sts = context
        .banks_client
        .get_account(acc.seller_trade_state)
        .await
        .expect("Error Getting Trade State")
        .expect("Trade State Empty");
    assert_eq!(sts.data.len(), 1);

    let timestamp = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .unwrap()
        .unix_timestamp;

    let listing_receipt_account = context
        .banks_client
        .get_account(listing_receipt_acc.receipt)
        .await
        .expect("getting listing receipt")
        .expect("empty listing receipt data");

    let listing_receipt =
        ListingReceipt::try_deserialize(&mut listing_receipt_account.data.as_ref()).unwrap();

    assert_eq!(listing_receipt.auction_house, acc.auction_house);
    assert_eq!(listing_receipt.metadata, acc.metadata);
    assert_eq!(listing_receipt.seller, acc.wallet);
    assert_eq!(listing_receipt.created_at, timestamp);
    assert_eq!(listing_receipt.purchase_receipt, None);
    assert_eq!(listing_receipt.canceled_at, None);
    assert_eq!(listing_receipt.bookkeeper, *owner_pubkey);
    assert_eq!(listing_receipt.seller, *owner_pubkey);
    assert_eq!(listing_receipt.price, 1);
    assert_eq!(listing_receipt.token_size, 6);
}
