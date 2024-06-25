use std::io::prelude::Read;
use std::io::{BufReader, BufRead, Cursor, Seek, SeekFrom};

use crate::BootSector;

struct Fat12_16 {
    bs_drvnum:          u8,
    bs_reserved1:       u8,
    bs_bootsig:         u8,
    bs_volid:           u32,
    bs_vollab:          [u8; 11],
    bs_filsystype:      [u8; 8],
    boot_program:       [u8; 448],
}
impl Fat12_16 {
    fn new() -> Self {
        Fat12_16 {
            bs_drvnum:      0u8,
            bs_reserved1:   0u8,
            bs_bootsig:     0u8,
            bs_volid:       0u32,
            bs_vollab:      [0u8; 11],
            bs_filsystype:  [0u8; 8],
            boot_program:   [0u8; 448],
        }
    }
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

impl Fat32 {
    fn new() -> Self {
        Fat32 {
            bpb_fatsz32:    0u32,
            bpb_extflags:   0u16,
            bpb_fsver:      [0u8; 2],
            bpb_rootclus:   0u32,
            bpb_fsinfo:     0u16,
            bpb_bkbootsec:  0u16,
            bpb_reserved:   [0u8; 12],
            bs_drvnum:      0u8,
            bs_reserved1:   0u8,
            bs_bootsig:     0u8,
            bs_volid:       0u32,
            bs_vollab:      [0u8; 11],
            bs_filsystype:  [0u8; 8],
            boot_program:   [0u8; 420],
        }
    }
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
    fat12_16: Fat12_16, // 36 - 509
    fat32: Fat32,       // 36 - 509
    last_signature: [u8; 2],
}

impl PBRFat {
    fn new(data: &[u8; 512]) -> Self {
        let mut cur = Cursor::new(data);
        let mut bs_jmpboot = [0u8; 3];
        cur.read_exact(&mut bs_jmpboot).unwrap();
        let mut bs_oemname = [0u8; 8];
        cur.read_exact(&mut bs_oemname).unwrap();
        let mut _bpb_bytspersec = [0u8; 2];
        cur.read_exact(&mut _bpb_bytspersec).unwrap();
        let bpb_bytspersec = u16::from_be_bytes(_bpb_bytspersec);
        let mut bpb_secperclus = 0u8;
        cur.read_exact(std::slice::from_mut(&mut bpb_secperclus)).unwrap();
        let mut _bpb_rsvdseccnt = [0u8; 2];
        cur.read_exact(&mut _bpb_rsvdseccnt).unwrap();
        let bpb_rsvdseccnt = u16::from_be_bytes(_bpb_rsvdseccnt);
        let mut bpb_numfats = 0u8;
        cur.read_exact(std::slice::from_mut(&mut bpb_numfats)).unwrap();
        let mut _bpb_rootentcnt = [0u8; 2];
        cur.read_exact(&mut _bpb_rootentcnt).unwrap();
        let bpb_rootentcnt = u16::from_be_bytes(_bpb_rootentcnt);
        let mut _bpb_totsec16 = [0u8; 2];
        cur.read_exact(&mut _bpb_totsec16).unwrap();
        let bpb_totsec16 = u16::from_be_bytes(_bpb_totsec16);
        let mut bpb_media = 0u8;
        cur.read_exact(std::slice::from_mut(&mut bpb_media)).unwrap();
        let mut _bpb_fatsz16 = [0u8; 2];
        cur.read_exact(&mut _bpb_fatsz16).unwrap();
        let bpb_fatsz16 = u16::from_be_bytes(_bpb_fatsz16);
        let mut _bpb_secpertrk = [0u8; 2];
        cur.read_exact(&mut _bpb_secpertrk).unwrap();
        let bpb_secpertrk = u16::from_be_bytes(_bpb_secpertrk);
        let mut _bpb_numheads = [0u8; 2];
        cur.read_exact(&mut _bpb_numheads).unwrap();
        let bpb_numheads = u16::from_be_bytes(_bpb_numheads);
        let mut _bpb_hiddsec = [0u8; 4];
        cur.read_exact(&mut _bpb_hiddsec).unwrap();
        let bpb_hiddsec = u32::from_be_bytes(_bpb_hiddsec);
        let mut _bpb_totsec32 = [0u8; 4];
        cur.read_exact(&mut _bpb_totsec32).unwrap();
        let bpb_totsec32 = u32::from_be_bytes(_bpb_totsec32);

        let mut fat12_16 = Fat12_16::new();
        cur.read_exact(std::slice::from_mut(&mut fat12_16.bs_drvnum)).unwrap();
        cur.read_exact(std::slice::from_mut(&mut fat12_16.bs_reserved1)).unwrap();
        cur.read_exact(std::slice::from_mut(&mut fat12_16.bs_bootsig)).unwrap();
        let mut _bs_volid = [0u8; 4];
        cur.read_exact(&mut _bs_volid).unwrap();
        fat12_16.bs_volid = u32::from_be_bytes(_bs_volid);
        cur.read_exact(&mut fat12_16.bs_vollab).unwrap();
        cur.read_exact(&mut fat12_16.bs_filsystype).unwrap();
        cur.read_exact(&mut fat12_16.boot_program).unwrap();
        
        cur.seek(SeekFrom::Start(36)).unwrap();

        let mut fat32 = Fat32::new();
        let mut _bpb_fatsz32 = [0u8; 4];
        cur.read_exact(&mut _bpb_fatsz32).unwrap();
        fat32.bpb_fatsz32 = u32::from_be_bytes(_bpb_fatsz32);
        let mut _bpb_extflags = [0u8; 2];
        cur.read_exact(&mut _bpb_extflags).unwrap();
        fat32.bpb_extflags = u16::from_be_bytes(_bpb_extflags);
        cur.read_exact(&mut fat32.bpb_fsver).unwrap();
        let mut _bpb_rootclus = [0u8; 4];
        cur.read_exact(&mut _bpb_rootclus).unwrap();
        fat32.bpb_rootclus = u32::from_be_bytes(_bpb_rootclus);
        let mut _bpb_fsinfo = [0u8; 2];
        cur.read_exact(&mut _bpb_fsinfo).unwrap();
        fat32.bpb_fsinfo = u16::from_be_bytes(_bpb_fsinfo);
        let mut _bpb_bkbootsec = [0u8; 2];
        cur.read_exact(&mut _bpb_bkbootsec).unwrap();
        fat32.bpb_bkbootsec = u16::from_be_bytes(_bpb_bkbootsec);
        cur.read_exact(&mut fat32.bpb_reserved).unwrap();
        cur.read_exact(std::slice::from_mut(&mut fat32.bs_drvnum)).unwrap();
        cur.read_exact(std::slice::from_mut(&mut fat32.bs_reserved1)).unwrap();
        cur.read_exact(std::slice::from_mut(&mut fat32.bs_bootsig)).unwrap();
        let mut _bs_volid = [0u8; 4];
        cur.read_exact(&mut _bs_volid).unwrap();
        fat32.bs_volid = u32::from_be_bytes(_bs_volid);
        cur.read_exact(&mut fat32.bs_vollab).unwrap();
        cur.read_exact(&mut fat32.bs_filsystype).unwrap();
        cur.read_exact(&mut fat32.boot_program).unwrap();
        let mut last_signature = [0u8; 2];
        cur.read_exact(&mut last_signature).unwrap();
        PBRFat {
            bs_jmpboot,
            bs_oemname,
            bpb_bytspersec,
            bpb_secperclus,
            bpb_rsvdseccnt,
            bpb_numfats,
            bpb_rootentcnt,
            bpb_totsec16,
            bpb_media,
            bpb_fatsz16,
            bpb_secpertrk,
            bpb_numheads,
            bpb_hiddsec,
            bpb_totsec32,
            fat12_16,
            fat32,
            last_signature,
        }
    }
}

impl BootSector for PBRFat {
    fn print_info(&self) {
        unimplemented!()
    }
    fn print_asm(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pbrfat_new(){
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
        let pbrfat = PBRFat::new(&data);
        assert_eq!(pbrfat.bs_jmpboot, data[0..3]);
        assert_eq!(pbrfat.bs_oemname, data[3..11]);
        let tmp: [u8; 2] = (&data[11..13]).try_into().unwrap();
        assert_eq!(pbrfat.bpb_bytspersec, u16::from_be_bytes(tmp));
    }
}
