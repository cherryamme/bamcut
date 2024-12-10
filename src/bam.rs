// use flate2::read;
use log::{debug, error, info};
use rust_htslib::bam::{self, ext::BamRecordExtensions, Read, Record};
use rust_htslib::bcf::record::Info;
use std::fs::create_dir_all;
use std::io::BufWriter;
use std::{fmt, str::FromStr};
// use std::collections::HashMap;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::io::Write;


#[derive(Debug, Clone)]
struct Region {
    chrom: String,
    start: i32,
    end: i32,
}
impl Region {
    pub fn new(chrom: String, start: i32, end: i32) -> Self {
        Self { chrom, start, end }
    }
}
impl std::str::FromStr for Region {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let chrom = parts[0].to_owned();
        let parts: Vec<&str> = parts[1].split('-').collect();
        let start = parts[0].parse()?;
        let end = parts[1].parse()?;
        Ok(Region { chrom, start, end })
    }
}

#[derive(Debug, Clone)]
pub struct RecordInfo {
    pub record: Record,
    pub cut_seq: String,
    pub cut_qual: String,
}
impl RecordInfo {
    fn info(&self) -> String {
        format!(
            "@{}\n{}\n+\n{}\n",
            std::str::from_utf8(self.record.qname()).unwrap(),
            self.cut_seq,
            self.cut_qual
        )
    }
}

pub fn cut_region_reads(record: &Record, region: &Region) -> Option<RecordInfo> {
    let start_pos = match record.cigar().read_pos(region.start as u32, true, true) {
        Ok(Some(pos)) => Some(pos as usize),
        _ => None,
    };
    let end_pos = match record.cigar().read_pos(region.end as u32, true, true) {
        Ok(Some(pos)) => Some(pos as usize),
        _ => None,
    };

    if start_pos.is_none() || end_pos.is_none() {
        return None;
    }
    let start_pos = start_pos.unwrap();
    let end_pos = end_pos.unwrap();

    let qual = record.qual()[start_pos as usize..end_pos as usize]
        .iter()
        .map(|&q| q + 33)
        .collect::<Vec<u8>>();
    let seq = &record.seq().as_bytes()[start_pos as usize..end_pos as usize];
    let recordinfo = RecordInfo {
        record: record.clone(),
        cut_seq: std::str::from_utf8(seq).unwrap().to_owned(),
        cut_qual: std::str::from_utf8(&qual).unwrap().to_owned(),
    };
    debug!("record: {:?}", recordinfo.info());
    Some(recordinfo)
}

// NOTE 读取bam文件的aligned_pairs
// let infos = record.aligned_pairs();
// for i in infos {
//     debug!("i: {:?}", i);
// }

pub fn process_bam(file: String, outfile: String, region: String, max_reads: Option<usize>) {
    let start_time = Instant::now();
    let mut writer = get_writer(outfile);
    let mut record_count = 0;
    let mut cut_record_count = 0;
    // 解析需要提取的region
    let region = Region::from_str(&region).unwrap();
    // 加载bam文件
    let mut bam = bam::IndexedReader::from_path(&file).unwrap();
    // 读取bam文件的index
    let header = bam::Header::from_template(bam.header());
    // log日志，打印bam文件的header信息
    // info!("Process bam: {:?}\nHeader: {:?}", file, header);
    bam.fetch((&region.chrom, region.start, region.end))
        .unwrap();
    // 循环处理bam文件中的每一行
    for read in bam.records() {
        if let Some(max) = max_reads {
            if cut_record_count >= max {
                break;
            }
        }
        record_count += 1;
        if let Some(recordinfo) = cut_region_reads(&read.unwrap(), &region){
            // debug!("read cigar: {:?}", &read.unwrap().pos());
            cut_record_count += 1;
            writer_recordinfo(recordinfo, &mut writer);
        }

    }
    info!(
        "Process bam: {:?} finished",
        file,
        );
        info!("cut {:?}/{:?} records in {:?}", cut_record_count, record_count, region);
    info!("total time:{:?} ", start_time.elapsed());
}

fn get_writer(outfile: String) -> BufWriter<GzEncoder<File>> {
    let filepath = Path::new(&outfile);
    let filedir = &filepath.parent().unwrap();
    create_dir_all(&filedir).expect("fail to create output directory");
    let file = File::create(&filepath).expect("fail to create output fq.gz");
    let encoder = GzEncoder::new(file, Compression::default());
    let mut writer = BufWriter::with_capacity(1_000_000, encoder);
    writer
}

pub fn writer_recordinfo(recordinfo: RecordInfo, writer: &mut BufWriter<GzEncoder<File>> ) {

    write!(writer, "{}", recordinfo.info()).unwrap();
}
