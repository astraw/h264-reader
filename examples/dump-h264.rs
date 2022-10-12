use std::io::Read;

use h264_reader::annexb::AnnexBReader;
use h264_reader::nal::{Nal, RefNal, UnitType};
use h264_reader::push::NalInterest;

fn main() {
    let mut calls = Vec::new();
    let mut reader = AnnexBReader::accumulate(|nal: RefNal<'_>| {
        let nal_unit_type = nal.header().unwrap().nal_unit_type();
        calls.push((nal_unit_type, nal.is_complete()));
        match nal_unit_type {
            UnitType::SeqParameterSet => NalInterest::Buffer,
            _ => NalInterest::Ignore,
        }
    });

    let fname = std::env::args()
        .skip(1)
        .next()
        .expect("expected h264 filename as command-line argument");
    let mut fd = std::fs::File::open(&fname).expect(&format!("while opening {fname}"));

    let mut buf = vec![0u8; 1024];
    loop {
        let sz = fd.read(&mut buf).unwrap();
        if sz == 0 {
            break;
        }
        reader.push(&buf[..sz]);
    }
    for call in calls.iter() {
        println!("{call:?}");
    }
}
