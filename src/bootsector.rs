use std::io::prelude::Read;
use std::io::{BufReader, BufRead, Cursor};

#[derive(Debug, PartialEq)]
pub enum BootSectorKind {
    MBR,
    PBRFat,
    Unknown,
}

pub trait BootSector {
    fn print_info();
    fn print_asm();
}

#[derive(Copy, Clone, Debug)]
struct Partition {
    active: u8,
    chs_start: [u8; 3],
    kind: u8,
    chs_end: [u8; 3],
    lba_start: u32,
    size: u32
}
impl Partition {
    fn new() -> Self {
        Partition {
            active: 0u8,
            chs_start: [0u8; 3],
            kind: 0u8,
            chs_end: [0u8; 3],
            lba_start: 0u32,
            size: 0u32,
        }
    }
}

#[derive(Debug)]
pub struct MBR{
    boot_code: [u8; 446],
    partition_table: [Partition; 4],
    boot_signature: [u8; 2],
}

impl MBR {
    fn new(data: &[u8; 512]) -> Self {
        let mut cur = Cursor::new(data);
        let mut boot_code = [0u8; 446];
        cur.read_exact(&mut boot_code).unwrap();
        let mut partition_table = [Partition::new(); 4];
        for partition in partition_table.iter_mut() {
            cur.read_exact(std::slice::from_mut(&mut partition.active)).unwrap();
            cur.read_exact(&mut partition.chs_start).unwrap();
            cur.read_exact(std::slice::from_mut(&mut partition.kind)).unwrap();
            cur.read_exact(&mut partition.chs_end).unwrap();
            let mut _lba_start = [0u8; 4];
            cur.read_exact(&mut _lba_start).unwrap();
            partition.lba_start = u32::from_be_bytes(_lba_start);
            let mut _size = [0u8; 4];
            cur.read_exact(&mut _size).unwrap();
            partition.size = u32::from_be_bytes(_size);
        }
        let mut boot_signature = [0u8; 2];
        cur.read_exact(&mut boot_signature).unwrap();
        MBR {
            boot_code,
            partition_table,
            boot_signature,
        }
    }
}
struct Fat12_16 {
    bs_drvnum:          u8,
    bs_reserved1:       u8,
    bs_bootsig:         u8,
    bs_volid:           u32,
    bs_vollab:          [u8; 11],
    bs_filsystype:      [u8; 8],
    boot_program:       [u8; 448],
}

struct Fat32 {
    bpb_fatsz32:    u32,
    bpb_extflags:   u16,
    bpb_fsver:      [u8; 2],
    bpb_rootclus:   u32,
    bpb_fsinfo:     u16,
    bpb_bkbootsec:  u16,
    bpb_reserved:   [u8; 12],
    bs_drvnum:      u8,
    bs_reserved1:   u8,
    bs_bootsig:     u8,
    bs_volid:       u32,
    bs_vollab:      [u8; 11],
    bs_filsystype:  [u8; 8],
    boot_program:   [u8; 420],
}

pub struct PBRFat{
    bs_jmpboot: [u8; 3],
    bs_oemname: [u8; 8],
    bpb_bytspersec: u16,
    bpb_secperclus: u8,
    bpb_rsvdseccnt: u16,
    bpb_numfats:    u8,
    bpb_rootentcnt: u16,
    bpb_totsec16:   u16,
    bpb_media:      u8,
    bpb_fatsz16:    u16,
    bpb_secpertrk:  u16,
    bpb_numheads:   u16,
    bpb_hiddsec:    u32,
    bpb_totsec32:   u32,
    fat12_16: Fat12_16,
    fat32: Fat32,
    last_signature: [u8; 2],
}

pub fn infer(data: &[u8]) -> BootSectorKind {
    return BootSectorKind::Unknown;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bootsector_mbr() {
        let mut data = [0u8; 512];
        let mut b = 0u8;
        for d in data.iter_mut() {
            match b.checked_add(1) {
                Some(n) => {
                    *d = n;
                    b = n;
                },
                None => b = 0u8,
            }
        }
        let mbr = MBR::new(&data);
        assert_eq!(mbr.boot_code, data[0..446]);
        assert_eq!(mbr.boot_signature, data[510..512]);
    }

    #[test]
    fn test_infer(){
        unimplemented!();
        // let data: [u8; 512] = [0; 512];
        // assert_eq!(infer(&data), BootSectorKind::Unknown);
        // let data: [u8; 512] = [0; 512];
        // assert_eq!(infer(&data), BootSectorKind::MBR);
        // let data: [u8; 512] = [0; 512];
        // assert_eq!(infer(&data), BootSectorKind::PBRFat);
    }
}
