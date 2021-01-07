# What makes Solana the fastest public blockchain?

Solana is a blockchain platform for running smart contracts.
What makes it unique? Its speed!

- block creation time < 800ms
- Transaction per second up to 50K

You can see the stat in realtime at https://solanabeach.io/.

So how is it possible to al-ready achieve so much speed while a giant like Ethereum is yet far from scaling?

## Proof of History breakthrough:

The key to that speed is Proof of History.

Simply put it is proof that time has passed between two transactions/statements.\
It relies on performing a task that can only be computed on one core while being verifiable on many cores using
parallelization.

As you can only use one core to create the proof when you give the proof to a validator he now that you spend X CPU cycle to produce it. Therefore you must have to spent some time to create it.

### Let's break it down

It all start with a "random" string (here it's `"GENISIS_STRING"`)
Hash that string with sha256:

```
sha256(GENISIS_STRING) = 3969095c1b5b03fa8144d5fb9f0cca5b632875db982a1f6404b21c4d0f71e8f1
```

Now, let's take the result use as our input to iterate on it:

```
sha256(3969095c1b5b03fa8144d5fb9f0cca5b632875db982a1f6404b21c4d0f71e8f1) = 521f0afc4cbe6726bbac7aa554bff62081be9d77bf3044f6eae952f7348386d3
```

iterate again...

```
sha256(521f0afc4cbe6726bbac7aa554bff62081be9d77bf3044f6eae952f7348386d3) = 1314f1dcb8d49d8517f09d4c88bfe9aa72b0599a864d35e9d00d3bb60a2ed69c
```

You get the picture. Now you got a list of proof looking like that:

```json
[
  {
    "round": 1,
    "input": "GENISIS_STRING",
    "output": "3969095c1b5b03fa8144d5fb9f0cca5b632875db982a1f6404b21c4d0f71e8f1"
  },
  {
    "round": 2,
    "input": "3969095c1b5b03fa8144d5fb9f0cca5b632875db982a1f6404b21c4d0f71e8f1",
    "output": "521f0afc4cbe6726bbac7aa554bff62081be9d77bf3044f6eae952f7348386d3"
  },
  {
    "round": 3,
    "input": "521f0afc4cbe6726bbac7aa554bff62081be9d77bf3044f6eae952f7348386d3",
    "output": "1314f1dcb8d49d8517f09d4c88bfe9aa72b0599a864d35e9d00d3bb60a2ed69c"
  }
]
```

For demonstration purposes, let's say it takes 1sec to create a sha256 hash.
It would take me 3 sec to create this "proof".
If I give it to you. You could verify quicker that it took me to produce it (supposedly about 1 sec using 3 core).

Here is a example (in rust) of producing a stream of proof & verify it in parallel:

```rust
// create some proof of history
let mut sha256 = Sha256::new();
let mut hashes = Vec::new();
sha256.input_str("GENISIS_STRING");
println!("{:?}", hashes[0]);

for i in 1..1000000 {
    sha256 = Sha256::new();
    sha256.input_str(hashes[i - 1].as_str());
    hashes.push(sha256.result_str());
}

// now verify that proof in parallel...
let mut children = vec![];
let proofs = Arc::new(hashes);

for i in 0..10 {
    let val = proofs.clone();
    children.push(thread::spawn(move || {
        for j in i * 100000..i * 100000 + 100000 {
            let proof_str = if j == 0 {
                "GENISIS_STRING"
            } else {
                val[j - 1].as_str()
            };
            let mut hasher = Sha256::new();
            hasher.input_str(proof_str);

            let proof = val[j].as_str();
            let output = hasher.result_str();
            println!("output N°{:?}: {:?}", j, output);
            println!("proof N°{:?}: {:?}", j, proof);

            if proof != output {
                panic!("hello panic");
            }
        }
    }));
}
```

Full demo code available at:\
https://github.com/lsmod/proof-of-history-explained

As you can see, you could verify in fewer second it takes to produce the proof.
But the idea is not to know how many seconds passed, just that time passed.

### Ok, I get proof of history! What's the point?

Let's get back a bit and look at bitcoin consensus.\
Mining block is like brute-forcing to find the right nonce it takes to make the block valid. All miners try and try... And eventually one finds the correct nonce.

This process is random. We don't know which miner/node will produce the next block.\
Hell! Two nodes could even produce two valid blocks at the same time!

When a miner produces a block it has to propagate this block to other nodes and they eventually come to an agreement (by starting mining on this new block).\
That's for proof of Work.

### Now, what about Proof of Stake?

With proof of Stake no mining right?\
Stakers/node are block producers and got the right to produce X% of the block depending on the number of coins they stake. It allows for scheduling block production.\
Now one problem:

- **Producing a block costs nothing.** \
  Staker/node creates a block and tells the other it's the new one. But as it cost nothing. It's possible for a node to create X new blocks at the same time.\
  The purpose? Create a double-spending attack.
  Long-range attacks are also possible.

- **One other issue is information propagation time.**\
  If it is my turn to produce a block, but when
  I send it to other nodes there is a delay/interruption in transmission?\
  Did I create it in time?\
   What if you receive if after it was considered invalid by certain nodes but validate by others (whom received it in time)?
- **And what about bandwidth ?**
  Node have to transmit blocks to others as well as transactions. Many transactions, possibly several blocks at the same time. As it's P2P nodes repeat what they received form others as well as directly received transactions. As bandwidth is not unlimited that's a bottleneck.

Consensus has to cope with those and find clever ways to prevent these issues.

### Back to Proof of history.

Solana uses proof of history as a tool in its Proof of Stake consensus.
When a node produces a block, transactions & blocks are anchored in proof of history.

- **What if a block producer tries to create two blocks at the same time for a double-spending attack?**\
  Validators could observe that the validator created two blocks at the same time (and that it's not a network delay issue). Slashing could be used to punish such malicious behavior.

- **Long-range attack?**\
  As it takes time to produce a block, it should be near impossible to produce many blocks in advance without being late on the current block.

- **Rewriting history?**\
  It makes it time costly to rewrite previous blocks.

- **Block producing scheduling:**\
  Nodes can easily agree on whom turns to produce a block it is.
  This way, only one node transmit the block it has produced at the time!\
  It allow blocks propagation optimization (kind of looking like bitorrentmore about this at following link: https://medium.com/solana-labs/turbine-solanas-block-propagation-protocol-solves-the-scalability-trilemma-2ddba46a51db). \
  Finally, It also Makes it easier to punish a node that didn't do its job in time.

## Conclusion:

I hope I helped you get a better grasp of what makes Solana Uniq/Fast.
Proof of history is not the only reason but the main one.

You can read about other innovation of the project here:\
https://medium.com/solana-labs/7-innovations-that-make-solana-the-first-web-scale-blockchain-ddc50b1defda

This article aims to vulgarize Solana concept therefore stuff has been simplified. Please note that I'm by no mean a blockchain consensus mechanisms expert.

To finish: One video that helped me understand the most:\
https://www.youtube.com/watch?v=rKGhbC6Uync
