use web3::types::{TransactionRequest, U256, H160, H256, Bytes, Address};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let _ = env_logger::try_init();
    let transport = web3::transports::WebSocket::new("ws://localhost:8546").await?;
    let web3 = web3::Web3::new(transport);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }

    Ok(())
}

pub fn eth_address_from_str_to_H160(addr: &str) -> Address {
    let addr = hex::decode(addr).unwrap();
    H160::from_slice(&addr)
}


#[test]
fn balance_test() {
    let http_url = "https://goerli.infura.io/v3/c6d0f579f3f64c3ea21a934428ed0fe1";
    async_std::task::block_on(async move {
        let transport = web3::transports::WebSocket::new("ws://172.18.11.37:6790").await.unwrap();
        let http_url = "https://goerli.infura.io/v3/c6d0f579f3f64c3ea21a934428ed0fe1";

        let web3 = web3::Web3::new(transport);

        //account from
        let account_from = "0x0005DdDCCBd5AF0880564BCB6a3eA308B214FB50";
        let mut acc_from = eth_address_from_str_to_H160(&account_from[2..]);

        let balance_before = web3.eth().balance(acc_from, None).await;
        let balance_before = web3.eth().block_number().await;
//        let nonce = web3.eth().transaction_count(acc_from, None).await.unwrap();
//
//        let signed_tx = construct_tx(http_url ,account_from, "0x7a250d5630b4cf539739df2c5dacb4c659f2488d", 1_000_000_000_000_000_000 , nonce, data).await;
//        let tx_hash = web3.eth().send_raw_transaction(Bytes::from(signed_tx)).await;
//        let balance_after = web3.eth().balance(acc_from, None).await;
//        println!("TX Hash: {:?}", tx_hash);
        println!("Balance before: {:?}", balance_before);
     //   println!("Balance after: {:?}", balance_after);
    })
}

