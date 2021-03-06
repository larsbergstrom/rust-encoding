// AUTOGENERATED FROM index-ibm866.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-ibm866.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    1040, 1041, 1042, 1043, 1044, 1045, 1046, 1047, 1048, 1049, 1050, 1051,
    1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060, 1061, 1062, 1063,
    1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075,
    1076, 1077, 1078, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086, 1087,
    9617, 9618, 9619, 9474, 9508, 9569, 9570, 9558, 9557, 9571, 9553, 9559,
    9565, 9564, 9563, 9488, 9492, 9524, 9516, 9500, 9472, 9532, 9566, 9567,
    9562, 9556, 9577, 9574, 9568, 9552, 9580, 9575, 9576, 9572, 9573, 9561,
    9560, 9554, 9555, 9579, 9578, 9496, 9484, 9608, 9604, 9612, 9616, 9600,
    1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098, 1099,
    1100, 1101, 1102, 1103, 1025, 1105, 1028, 1108, 1031, 1111, 1038, 1118,
    176, 8729, 183, 8730, 8470, 164, 9632, 160,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        1040 => 128, 1041 => 129, 1042 => 130, 1043 => 131, 1044 => 132,
        1045 => 133, 1046 => 134, 1047 => 135, 1048 => 136, 1049 => 137,
        1050 => 138, 1051 => 139, 1052 => 140, 1053 => 141, 1054 => 142,
        1055 => 143, 1056 => 144, 1057 => 145, 1058 => 146, 1059 => 147,
        1060 => 148, 1061 => 149, 1062 => 150, 1063 => 151, 1064 => 152,
        1065 => 153, 1066 => 154, 1067 => 155, 1068 => 156, 1069 => 157,
        1070 => 158, 1071 => 159, 1072 => 160, 1073 => 161, 1074 => 162,
        1075 => 163, 1076 => 164, 1077 => 165, 1078 => 166, 1079 => 167,
        1080 => 168, 1081 => 169, 1082 => 170, 1083 => 171, 1084 => 172,
        1085 => 173, 1086 => 174, 1087 => 175, 9617 => 176, 9618 => 177,
        9619 => 178, 9474 => 179, 9508 => 180, 9569 => 181, 9570 => 182,
        9558 => 183, 9557 => 184, 9571 => 185, 9553 => 186, 9559 => 187,
        9565 => 188, 9564 => 189, 9563 => 190, 9488 => 191, 9492 => 192,
        9524 => 193, 9516 => 194, 9500 => 195, 9472 => 196, 9532 => 197,
        9566 => 198, 9567 => 199, 9562 => 200, 9556 => 201, 9577 => 202,
        9574 => 203, 9568 => 204, 9552 => 205, 9580 => 206, 9575 => 207,
        9576 => 208, 9572 => 209, 9573 => 210, 9561 => 211, 9560 => 212,
        9554 => 213, 9555 => 214, 9579 => 215, 9578 => 216, 9496 => 217,
        9484 => 218, 9608 => 219, 9604 => 220, 9612 => 221, 9616 => 222,
        9600 => 223, 1088 => 224, 1089 => 225, 1090 => 226, 1091 => 227,
        1092 => 228, 1093 => 229, 1094 => 230, 1095 => 231, 1096 => 232,
        1097 => 233, 1098 => 234, 1099 => 235, 1100 => 236, 1101 => 237,
        1102 => 238, 1103 => 239, 1025 => 240, 1105 => 241, 1028 => 242,
        1108 => 243, 1031 => 244, 1111 => 245, 1038 => 246, 1118 => 247,
        176 => 248, 8729 => 249, 183 => 250, 8730 => 251, 8470 => 252,
        164 => 253, 9632 => 254, 160 => 255, _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(128, 256) {
            let i = i as u8;
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}
