mod transaction;

use transaction::types::Transaction;
use transaction::processor::{Bank, Processor};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<Transaction>();

    // Shared Bank state
    let mut bank = Bank::new();

    // Spawn processing thread
    let handle = thread::spawn(move || {
        while let Ok(transaction) = rx.recv() {
            let mut processor: Box<dyn Processor> = Box::new(bank.clone());
            processor.process(transaction);
        }
    });

    // Send transactions
    tx.send(Transaction::Deposit { account_id: 1, amount: 1000 }).unwrap();
    tx.send(Transaction::Withdraw { account_id: 1, amount: 200 }).unwrap();
    tx.send(Transaction::Transfer { from: 1, to: 2, amount: 300 }).unwrap();

    // Give time for processing
    thread::sleep(Duration::from_millis(100));

    // End program
    handle.join().unwrap();
}
