mod common;

use serial_test::serial;

use crate::common:: {
    Api, Block, BlockHash, ServerBuilder, Transaction, ALICE, BLOCK_SUBSIDY, BOB, MINER_ADDRESS,
};

#[test]
#[serial]
#[cfg(unix)]
fn test_should_get_a_valid_genesis_block() {
    let node = ServerBuilder::new().start();

    // List the blocks by querying the REST API
    let blocks = node.get_blocks();

    // check that the blocks only contain the genesis block
    assert_eq!(blocks.len(), 1);
    let genesis_block = blocks.first().unwrap();

    // check that the genesis block fields are valid
    assert_eq!(genesis_block.index, 0);
    assert_eq!(genesis_block.nonce, 0);
    assert_eq!(genesis_block.previous_hash, BlockHash::default());
    assert!(genesis_block.transaction.is_empty());
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_let_add_transaction() {
    let mut node = ServerBuilder::new().start();
    let genesis_block = node.get_last_block();

    // create and add a new transaction to the pool
    // the sender must mining address
    // as it should have funds from the coinbase rewards of the genesis block
    let transaction = Transaction {
        sender: MINER_ADDRESS.to_string(),
        recipient: BOB.to_string(),
        amount: 10 as u64,
    };

    let res = node.add_transaction(&transaction);
    assert_eq!(res.status().as_u16(), 200);

    // wait for the transaction to be mined
    node.wait_for_mining();

    // check that a new block was added...
    let blocks = node.get_blocks();
    assert_eq!(blocks.len(), 2);
    let mined_block = blocks.last().unwrap();

    // valid
    assert_eq!(mined_block.index, 1);
    assert_eq!(mined_block.previous_hash, genesis_block.hash);

    // contains the transactions that we added
    assert_eq!(mined_block.transaction.len(), 2);
    let mined_transaction = mined_block.transaction.last().unwrap();
    assert_eq!(*mined_transaction, transaction);
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_let_add_valid_block() {
    let node = SeverBuilder::new().start();
    let geneis_block = node.get_last_block();
    let coinbase = Transaction {
        sender: ALICE.to_string(),
        recipient: ALICE.to_string(),
        amount: BLOCK_SUBSIDY,
    };

    let valid_block = Block {
        // there is the genesis block already, so the next index is 1
        index: 1,
        timestamp: 0,
        nonce: 0,

        // the previous hash is checked
        previous_hash: genesis_block.hash,
        // the api automatically recalculates the hash -> add a valid one
        hash: BlockHash::default(),
        // include the coinbase transaction.
        transactions: vec![coinbase],
    };

    let res  = node.add_block(&valid_block);
    assert_eq!(res.status().as_u16(), 200);
}

#[test]
#[serial]
#[cfg(unix)]
fn test_should_not_let_add_invalid_block() {
    let node = ServerBuilder::new().start();

    // try to add a new INVALID block, should return an error
    let invalid_block  = Block {
        index: 0, // not valid index, the genesis block already has index 0
        timestamp: 0,
        nonce: 0,
        previous_hash: BlockHash::default(), // also not valid
        hash: BlockHash::default(),
        transactions: [].to_vec(),
    };

    let res = node.add_block(&invalid_block);
    assert_eq!(res.status().as_u16(), 400);
}