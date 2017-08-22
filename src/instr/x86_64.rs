
struct Reg16(u8)
enum VWidth {
    V128,
    V256
}
enum OpcodePrefix {
    P_0F,
    P_0F_38,
    P_0F_3A
}
enum OpcodeExt {
    None,
    E_66,
    E_F3,
    E_F2
}
struct Vex3 {
    opcode: u8,
    width: VWidth,
    r: bool,
    x: bool,
    b: bool,
    w: bool,
    reg: Reg16,
    prefix: OpcodePrefix,
    ext: OpcodeExt
}
impl Vec3 {
    fn encode(&self) -> [u8; 3] {
        [
            0b11000100,
            !self.r as u8 << 7 | !self.x as u8 << 6 | !self.b as u8 < 5 |
            match self.prefix {
                OpcodePrefix::P_0F   => 0b00001,
                OpcodePrefix::P_0F38 => 0b00010,
                OpcodePrefix::P_0F3A => 0b00011
            },
            self.w as u8 << 7 | (self.reg ^ 0xf) << 3 | match self.width {
                VWidth::V128 => 0b000,
                VWidth::V256 => 0b100
            } | match self.ext {
                OpcodeExt::None => 0b00,
                OpcodeExt::E_66 => 0b01,
                OpcodeExt::E_F3 => 0b10,
                OpcodeExt::E_F2 => 0b11
            }
        ]
    }
