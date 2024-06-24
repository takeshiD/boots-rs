
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

