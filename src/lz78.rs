#[derive(PartialEq, Debug)]
pub struct Block(usize, Option<u8>);

pub fn compress(input_bytes: impl AsRef<[u8]>) -> Vec<Block> {
    let input_stream = input_bytes.as_ref();

    let mut dictionary: Vec<Block> = Vec::new();
    dictionary.push(Block(0, None));

    let mut coding_positon = 0;
    let mut last_matching_index = 0;

    while coding_positon < input_stream.len() {
        let current_char = input_stream[coding_positon];

        match dictionary.iter().enumerate().find(|(_, &ref block)| {
            if block.0 != last_matching_index {
                return false;
            }

            if let Some(literal) = block.1 {
                return literal == current_char;
            } else {
                return false;
            }
        }) {
            Some((index, _)) => {
                last_matching_index = index;
            }
            None => {
                dictionary.push(Block(last_matching_index, Some(current_char)));
                last_matching_index = 0;
            }
        }

        coding_positon += 1;
    }

    if last_matching_index != 0 {
        dictionary.push(Block(last_matching_index, None));
    }

    return dictionary;
}

pub fn decompress(blocks: &[Block]) -> Vec<u8> {
    let mut decompressed_data: Vec<u8> = Vec::new();
    let mut decompressed_chunk: Vec<u8> = Vec::new();

    let mut stack = Vec::new();
    stack.push(&blocks[0]);

    let mut block_counter = 1;

    while stack.len() > 0 {
        let current_block = stack.pop().unwrap();

        match &current_block.1 {
            Some(current_token) => {
                decompressed_chunk.push(*current_token);
                stack.push(&blocks[current_block.0]);
            },
            None => {
                decompressed_chunk.reverse();
                decompressed_data.append(&mut decompressed_chunk);
                if block_counter < blocks.len() {
                    stack.push(&blocks[block_counter]);
                    block_counter += 1;
                }
            },
        }
    }

    return decompressed_data;
}

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn compress() {
        let compressed_blocks = super::compress("ababcbababaa");

        let expected_output = vec![
            Block(0, None),
            Block(0, Some(97)),
            Block(0, Some(98)),
            Block(1, Some(98)),
            Block(0, Some(99)),
            Block(2, Some(97)),
            Block(5, Some(98)),
            Block(1, Some(97)),
        ];

        assert_eq!(expected_output, compressed_blocks);
    }

    #[test]
    fn decompress() {
        let decompressed_bytes = super::decompress(&[
            Block(0, None),
            Block(0, Some(97)),
            Block(0, Some(98)),
            Block(1, Some(98)),
            Block(0, Some(99)),
            Block(2, Some(97)),
            Block(5, Some(98)),
            Block(1, Some(97)),
        ]);

        assert_eq!("ababcbababaa".as_bytes(), decompressed_bytes);
    }
}
