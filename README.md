# Processor

## The Problem

Given a blackbox of monolithic data (looking from the outside, but comprised of well-defined blocks internally),
how do we process it efficiently using all compute power at our disposal?

## The Steps

1. Split the data to chunk, [0, N1], [N1, N2], ... [Nn, Nn+1].


2. When starting to process each chunk, first lock on the previous/next block.


3. Process the chunk and store relevant information to enable monolithic context.

```rust
struct Chunk {
    start: u64,
    end: u64,
    first_block_offset: u64,
    last_block_offset: u64,
    results: Vec<Block>,
}

struct Block {
    relative_offset: u64,
    data: Vec<u8>,
}
```

4. In case our monolithic data allows efficient random access,
   we can traverse backwards to ensure each block covers the expected range.
   If not, post-processing will require to detect if some small boundary blocks are missing.


5. Combine the results using the offset information.