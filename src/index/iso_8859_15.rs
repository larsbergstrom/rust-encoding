// AUTOGENERATED FROM index-iso-8859-15.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-iso-8859-15.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 161, 162, 163, 8364, 165, 352, 167, 353, 169, 170, 171, 172,
    173, 174, 175, 176, 177, 178, 179, 381, 181, 182, 183, 382, 185, 186, 187,
    338, 339, 376, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
    203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217,
    218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232,
    233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247,
    248, 249, 250, 251, 252, 253, 254, 255,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        128 => 128, 129 => 129, 130 => 130, 131 => 131, 132 => 132, 133 => 133,
        134 => 134, 135 => 135, 136 => 136, 137 => 137, 138 => 138, 139 => 139,
        140 => 140, 141 => 141, 142 => 142, 143 => 143, 144 => 144, 145 => 145,
        146 => 146, 147 => 147, 148 => 148, 149 => 149, 150 => 150, 151 => 151,
        152 => 152, 153 => 153, 154 => 154, 155 => 155, 156 => 156, 157 => 157,
        158 => 158, 159 => 159, 160 => 160, 161 => 161, 162 => 162, 163 => 163,
        8364 => 164, 165 => 165, 352 => 166, 167 => 167, 353 => 168,
        169 => 169, 170 => 170, 171 => 171, 172 => 172, 173 => 173, 174 => 174,
        175 => 175, 176 => 176, 177 => 177, 178 => 178, 179 => 179, 381 => 180,
        181 => 181, 182 => 182, 183 => 183, 382 => 184, 185 => 185, 186 => 186,
        187 => 187, 338 => 188, 339 => 189, 376 => 190, 191 => 191, 192 => 192,
        193 => 193, 194 => 194, 195 => 195, 196 => 196, 197 => 197, 198 => 198,
        199 => 199, 200 => 200, 201 => 201, 202 => 202, 203 => 203, 204 => 204,
        205 => 205, 206 => 206, 207 => 207, 208 => 208, 209 => 209, 210 => 210,
        211 => 211, 212 => 212, 213 => 213, 214 => 214, 215 => 215, 216 => 216,
        217 => 217, 218 => 218, 219 => 219, 220 => 220, 221 => 221, 222 => 222,
        223 => 223, 224 => 224, 225 => 225, 226 => 226, 227 => 227, 228 => 228,
        229 => 229, 230 => 230, 231 => 231, 232 => 232, 233 => 233, 234 => 234,
        235 => 235, 236 => 236, 237 => 237, 238 => 238, 239 => 239, 240 => 240,
        241 => 241, 242 => 242, 243 => 243, 244 => 244, 245 => 245, 246 => 246,
        247 => 247, 248 => 248, 249 => 249, 250 => 250, 251 => 251, 252 => 252,
        253 => 253, 254 => 254, 255 => 255, _ => 0
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
