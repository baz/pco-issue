use std::vec;

use pco::{standalone::FileCompressor, ChunkConfig};

fn main() {
    println!("Hello, world!");
    let compressor = FileCompressor::default();
    let mut result = Vec::new();
    compressor.write_header(&mut result).unwrap();
    let chunk = vec![167772161, 150994945, 167772161];
    let config = ChunkConfig::default();
    let chunk_compressor = compressor.chunk_compressor::<i32>(&chunk, &config).unwrap();
    chunk_compressor.write_chunk(&mut result);
    compressor.write_footer(&mut result);
}
