use std::io::Read;

use h264_reader::annexb::AnnexBReader;
use h264_reader::nal::slice::SliceHeader;
use h264_reader::nal::{Nal, RefNal, UnitType};
use h264_reader::push::NalInterest;

fn main() {
    let mut parsing_ctx = h264_reader::Context::default();
    let mut reader = AnnexBReader::accumulate(|nal: RefNal<'_>| {
        if nal.is_complete() {
            let nal_unit_type = nal.header().unwrap().nal_unit_type();

            dbg!(nal_unit_type);
            match nal_unit_type {
                UnitType::SeqParameterSet => {
                    let sps =
                        h264_reader::nal::sps::SeqParameterSet::from_bits(nal.rbsp_bits()).unwrap();
                    // println!("{sps:?}");
                    dbg!(&sps);
                    parsing_ctx.put_seq_param_set(sps);
                }
                UnitType::PicParameterSet => {
                    let pps = h264_reader::nal::pps::PicParameterSet::from_bits(
                        &parsing_ctx,
                        nal.rbsp_bits(),
                    )
                    .unwrap();
                    // println!("{pps:?}");
                    dbg!(&pps);
                    parsing_ctx.put_pic_param_set(pps);
                }
                UnitType::SliceLayerWithoutPartitioningIdr
                | UnitType::SliceLayerWithoutPartitioningNonIdr => {
                    match SliceHeader::from_bits(
                        &parsing_ctx,
                        &mut nal.rbsp_bits(),
                        nal.header().unwrap(),
                    ) {
                        Err(e) => {
                            panic!("SliceHeaderError: {:?}", e);
                        }
                        Ok((slice_header, _sps, _pps)) => {
                            // println!("{slice_header:?}");
                            dbg!(&slice_header);
                        }
                    }
                }
                _ => {} // ignore
            }
        }
        NalInterest::Buffer
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
}
