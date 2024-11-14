# Yet Another Lempel-Ziv

Various Lempel-Ziv compression algorithms.

## LZ77

```rust
use yalz::lz77;

fn main() {
    // Search buffer size | Lookahead buffer size
    let compressor = lz77::LZ77Compressor::new(6, 6);
    let compressed_blocks = compressor.compress("ababcbababaa");

    // OR

    let compressed_blocks = lz77::compress("ababcbababaa", 6, 6);

    println!("{:?}", compressed_blocks);

    let decompressed_bytes = lz77::decompress(&compressed_blocks);
    println!("{:?}", decompressed_bytes);
}
```

## LZ78

```rust
use yalz::lz78;

fn main() {
    let compressed_blocks = lz78::compress("ababcbababaa");
    println!("{:?}", compressed_blocks);

    let decompressed_bytes = lz78::decompress(&compressed_blocks);
    println!("{:?}", decompressed_bytes);
}
```

## LZSS

`todo!()`

## LZW

`todo!()`

## LZMA

`todo!()`