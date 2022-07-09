# Solana Idempotent

Include the transaction instructions atomically with transactions when you want to guarantee they don't run twice.

This uses an on-chain bitmap that you create initially, that you then use in the later transactions. It is left up to the client to interpret which bits represent transactions.

One workflow can look like this

1. Estimate the total number of transactions you are going to run
2. Create a map account accomodating this length divided by 8
3. Loop over your data and build the transactions, in each one include a transaction to mark the bit, where the bit is the nth loop iteration
4. Save your transactions & asynchronously send them with no worries of a double spend and no worry about trying to confirm them
