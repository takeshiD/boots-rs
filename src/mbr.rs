use std::io::prelude::Read;
use std::io::{BufReader, BufRead, Cursor};

use crate::BootSector;

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
    pub fn new(data: &[u8; 512]) -> Self {
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

impl BootSector for MBR {
    fn print_info(&self) {
        println!("\x1b[1mBootSectorType:\x1b[0m MBR");
        for (i, part) in self.partition_table.iter().enumerate() {
            println!("\x1b[1mPartition {i}:\x1b[0m");
            println!("  {: <20}0x{:02X}", "Active flag", part.active);
            let cylinder = ((part.chs_start[1] & 0xc0 << 2) | part.chs_start[2]) as u16; // 10bit
            let head = part.chs_start[0]; // 8bit
            let sector = part.chs_start[1] & 0x3f; // 6bit
            println!("  {: <20}C:0x{cylinder:04X}, H:0x{head:02X}, S:0x{sector:02X}", "CHS Start");
            println!("  {: <20}{}", "Partition Type", part.kind);
            let cylinder = ((part.chs_end[1] & 0xc0 << 2) | part.chs_end[2]) as u16; // 10bit
            let head = part.chs_end[0]; // 8bit
            let sector = part.chs_end[1] & 0x3f; // 6bit
            println!("  {: <20}C:0x{cylinder:04X}, H:0x{head:02X}, S:0x{sector:02X}", "CHS End");
            println!("  {: <20}0x{:08X}", "LBA Start", part.lba_start);
            println!("  {: <20}0x{:08X}", "Number of Sectors", part.size);
        }
        println!("\x1b[1m{: <22}\x1b[0m0x{:02X}", "BootSignature:",u16::from_le_bytes(self.boot_signature));
    }
    fn print_asm(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mbr_new(){
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
}
