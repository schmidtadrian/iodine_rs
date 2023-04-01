/// fix 48 byte check value
/// first 16 bytes are:
///     4 times: 0000 0000,
///     4 times: 1111 1111,
///     4 times: 0101 0101,
///     4 times: 1010 1010
/// followed by 32 random bytes
pub const DOWN_CODEC_CHECK: [u8; 48] = [
    0o000, 0o000, 0o000, 0o000, 0o377, 0o377, 0o377, 0o377,
    0o125, 0o125, 0o125, 0o125, 0o252, 0o252, 0o252, 0o252,
    0o201, 0o143, 0o310, 0o322, 0o307, 0o174, 0o262, 0o027,
    0o137, 0o117, 0o316, 0o311, 0o111, 0o055, 0o122, 0o041,
	0o141, 0o251, 0o161, 0o040, 0o045, 0o263, 0o006, 0o163,
    0o346, 0o330, 0o104, 0o060, 0o171, 0o120, 0o127, 0o277
];
