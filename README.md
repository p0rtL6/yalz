# Yet Another Lempel-Ziv

Various Lempel-Ziv compression algorithms.


## LZ77

```rust
fn main() {
    use yalz::lz77;

    // Search buffer size | Lookahead buffer size
    let compressor = lz77::LZ77Compressor::new(6, 6);

    let compressed_blocks = compressor.compress("ababcbababaa");
    println!("{:?}", compressed_blocks);

    let decompressed_bytes = lz77::decompress(&compressed_blocks);
    println!("{:?}", decompressed_bytes);
}
```

## LZ78

`todo!()`

## LZSS

`todo!()`

## LZW

`todo!()`

## LZMA

`todo!()`