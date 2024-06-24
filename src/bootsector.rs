#[derive(Debug, PartialEq)]
pub enum BootSectorKind {
    MBR,
    PBRFat,
    Unknown,
}

pub trait BootSector {
    fn print_info(&self);
    fn print_asm(&self);
}

pub fn infer(data: &[u8; 512]) -> BootSectorKind {
    if data[0x01fe] == 0x55 && data[0x01ff] == 0xaa {
        if (data[0x00] == 0xeb && data[0x02] == 0x90) || data[0x00] == 0xe9
        {
            return BootSectorKind::PBRFat;
        }
        else if data[0x01be] == 0x00 || data[0x01be] == 0x80
        {
            return BootSectorKind::MBR;
        }
    }
    return BootSectorKind::Unknown;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_infer(){
        unimplemented!()
    }
}
