#[derive(PartialEq, Debug)]
pub enum Block {
    Pair((usize, usize)),
    Literal(u8),
}

pub struct LZ77Compressor {
    search_buffer_size: usize,
    lookahead_buffer_size: usize,
}

impl LZ77Compressor {
    pub fn new(search_buffer_size: usize, lookahead_buffer_size: usize) -> Self {
        LZ77Compressor {
            search_buffer_size,
            lookahead_buffer_size,
        }
    }

    pub fn compress(&self, input_bytes: impl AsRef<[u8]>) -> Vec<Block> {
        compress(
            input_bytes,
            self.search_buffer_size,
            self.lookahead_buffer_size,
        )
    }
}

pub fn compress(
    input_bytes: impl AsRef<[u8]>,
    search_buffer_size: usize,
    lookahead_buffer_size: usize,
) -> Vec<Block> {
    let input_stream = input_bytes.as_ref();

    let mut output_stream: Vec<Block> = Vec::new();
    let mut coding_position: usize = 0;

    while coding_position < input_stream.len() {
        let mut buffer_size: usize = 1;
        let mut search_offset: usize = 0;

        let mut matched_block = Block::Literal(input_stream[coding_position]);

        while (buffer_size + search_offset) as usize <= search_buffer_size
            && coding_position >= (buffer_size + search_offset) as usize
            && buffer_size as usize <= lookahead_buffer_size
            && (coding_position + buffer_size as usize) <= input_stream.len()
        {
            let search_buffer = &input_stream[coding_position
                - (buffer_size + search_offset) as usize
                ..coding_position - search_offset as usize];
            let lookahead_buffer =
                &input_stream[coding_position..coding_position + buffer_size as usize];

            if lookahead_buffer == search_buffer {
                matched_block = Block::Pair((buffer_size, buffer_size + search_offset));
                buffer_size += 1;

                search_offset = 0;
            } else {
                search_offset += 1;
            }
        }

        match matched_block {
            Block::Literal(_byte) => {
                coding_position += 1;
            }
            Block::Pair((length, offset)) => {
                let run_start_position = coding_position + length - offset;
                let initial_match = &input_stream[run_start_position..run_start_position + length];
                let mut run_multiplier = 1;

                while run_start_position + (length * run_multiplier) < input_stream.len() {
                    let next_chunk = &input_stream[run_start_position + (length * run_multiplier)
                        ..run_start_position + (length * run_multiplier) + length];
                    if next_chunk == initial_match {
                        run_multiplier += 1;
                    } else {
                        break;
                    }
                }

                matched_block = Block::Pair((length * run_multiplier, offset));

                coding_position += (length * run_multiplier) as usize;
            }
        }
        output_stream.push(matched_block)
    }
    return output_stream;
}

pub fn decompress(blocks: &[Block]) -> Vec<u8> {
    let mut decompressed_data: Vec<u8> = Vec::new();

    for block in blocks {
        match block {
            Block::Literal(literal) => {
                decompressed_data.push(*literal);
            }
            Block::Pair((length, offset)) => {
                let end = decompressed_data.len() - *offset as usize;
                for i in 0..*length {
                    decompressed_data.push(decompressed_data[end + i]);
                }
            }
        }
    }

    return decompressed_data;
}

#[cfg(test)]
mod tests {
    use super::{Block, LZ77Compressor};

    #[test]
    fn compress() {
        let lz77_compressor = LZ77Compressor::new(6, 6);
        let compressed_blocks = lz77_compressor.compress("ababcbababaa");

        let expected_output = vec![
            Block::Literal(97),
            Block::Literal(98),
            Block::Pair((2, 2)),
            Block::Literal(99),
            Block::Pair((3, 4)),
            Block::Pair((2, 2)),
            Block::Pair((1, 2)),
            Block::Pair((1, 1)),
        ];

        assert_eq!(expected_output, compressed_blocks);
    }

    #[test]
    fn decompress() {
        let decompressed_bytes = super::decompress(&[
            Block::Literal(97),
            Block::Literal(98),
            Block::Pair((2, 2)),
            Block::Literal(99),
            Block::Pair((3, 4)),
            Block::Pair((2, 2)),
            Block::Pair((1, 2)),
            Block::Pair((1, 1)),
        ]);

        assert_eq!("ababcbababaa".as_bytes(), decompressed_bytes);
    }

    #[test]
    fn length_overflow() {
        let lz77 = LZ77Compressor::new(6, 6);
        let compressed_blocks = lz77.compress("abaaaa");
        let decompressed_bytes = super::decompress(&compressed_blocks);

        assert_eq!("abaaaa".as_bytes(), decompressed_bytes);
    }
}
