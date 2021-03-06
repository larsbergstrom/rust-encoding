// AUTOGENERATED FROM index-windows-1250.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-windows-1250.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    8364, 129, 8218, 131, 8222, 8230, 8224, 8225, 136, 8240, 352, 8249, 346,
    356, 381, 377, 144, 8216, 8217, 8220, 8221, 8226, 8211, 8212, 152, 8482,
    353, 8250, 347, 357, 382, 378, 160, 711, 728, 321, 164, 260, 166, 167, 168,
    169, 350, 171, 172, 173, 174, 379, 176, 177, 731, 322, 180, 181, 182, 183,
    184, 261, 351, 187, 317, 733, 318, 380, 340, 193, 194, 258, 196, 313, 262,
    199, 268, 201, 280, 203, 282, 205, 206, 270, 272, 323, 327, 211, 212, 336,
    214, 215, 344, 366, 218, 368, 220, 221, 354, 223, 341, 225, 226, 259, 228,
    314, 263, 231, 269, 233, 281, 235, 283, 237, 238, 271, 273, 324, 328, 243,
    244, 337, 246, 247, 345, 367, 250, 369, 252, 253, 355, 729,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

pub fn backward(code: u16) -> u8 {
    match code {
        8364 => 128, 129 => 129, 8218 => 130, 131 => 131, 8222 => 132,
        8230 => 133, 8224 => 134, 8225 => 135, 136 => 136, 8240 => 137,
        352 => 138, 8249 => 139, 346 => 140, 356 => 141, 381 => 142,
        377 => 143, 144 => 144, 8216 => 145, 8217 => 146, 8220 => 147,
        8221 => 148, 8226 => 149, 8211 => 150, 8212 => 151, 152 => 152,
        8482 => 153, 353 => 154, 8250 => 155, 347 => 156, 357 => 157,
        382 => 158, 378 => 159, 160 => 160, 711 => 161, 728 => 162, 321 => 163,
        164 => 164, 260 => 165, 166 => 166, 167 => 167, 168 => 168, 169 => 169,
        350 => 170, 171 => 171, 172 => 172, 173 => 173, 174 => 174, 379 => 175,
        176 => 176, 177 => 177, 731 => 178, 322 => 179, 180 => 180, 181 => 181,
        182 => 182, 183 => 183, 184 => 184, 261 => 185, 351 => 186, 187 => 187,
        317 => 188, 733 => 189, 318 => 190, 380 => 191, 340 => 192, 193 => 193,
        194 => 194, 258 => 195, 196 => 196, 313 => 197, 262 => 198, 199 => 199,
        268 => 200, 201 => 201, 280 => 202, 203 => 203, 282 => 204, 205 => 205,
        206 => 206, 270 => 207, 272 => 208, 323 => 209, 327 => 210, 211 => 211,
        212 => 212, 336 => 213, 214 => 214, 215 => 215, 344 => 216, 366 => 217,
        218 => 218, 368 => 219, 220 => 220, 221 => 221, 354 => 222, 223 => 223,
        341 => 224, 225 => 225, 226 => 226, 259 => 227, 228 => 228, 314 => 229,
        263 => 230, 231 => 231, 269 => 232, 233 => 233, 281 => 234, 235 => 235,
        283 => 236, 237 => 237, 238 => 238, 271 => 239, 273 => 240, 324 => 241,
        328 => 242, 243 => 243, 244 => 244, 337 => 245, 246 => 246, 247 => 247,
        345 => 248, 367 => 249, 250 => 250, 369 => 251, 252 => 252, 253 => 253,
        355 => 254, 729 => 255, _ => 0
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
