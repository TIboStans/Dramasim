#![allow(unused)]

// Instruction function codes
pub const FC_HIA: isize = 11;
pub const FC_BIG: isize = 12;
pub const FC_OPT: isize = 21;
pub const FC_AFT: isize = 22;
pub const FC_VER: isize = 23;
pub const FC_DEL: isize = 24;
pub const FC_MOD: isize = 25;
pub const FC_VGL: isize = 31;
pub const FC_SPR: isize = 32;
pub const FC_VSP: isize = 33;
pub const FC_SBR: isize = 41;
pub const FC_KTG: isize = 42;
pub const FC_LEZ: isize = 71;
pub const FC_DRU: isize = 72;
pub const FC_NWL: isize = 73;
pub const FC_DRS: isize = 74;
pub const FC_STP: isize = 99;

// Modus
pub const MOD1_VALUE: isize = 1;
pub const MOD1_VALUE_MOD: isize = 2;
pub const MOD1_ADDRESS: isize = 3;
pub const MOD1_INDIRECT_ADDRESS: isize = 4;

pub const MOD2_NO_INDEXATION: isize = 1;
pub const MOD2_INDEXATION: isize = 2;
pub const MOD2_INDEXATION_PRE_INC: isize = 3;
pub const MOD2_INDEXATION_PRE_DEC: isize = 4;
pub const MOD2_INDEXATION_POST_INC: isize = 5;
pub const MOD2_INDEXATION_POST_DEC: isize = 6;

// General
/// Not applicable
pub const NA: isize = 9;