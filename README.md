# Solana Idempotent

Include the transaction instructions atomically with transactions when you want to guarantee they don't run twice.

This uses an on-chain bitmap that you create initially, that you then use in the later transactions.
