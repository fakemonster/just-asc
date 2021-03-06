//! Out-of-the-box tilesets for your [GridConfig](crate::GridConfig).

/// Actual ASCII characters. Nice and crunchy.
///
/// ```text
///            __d""""""""""""b__
///         _P"`                '"¶_
///      .d"`                      '"b,
///     d"                            "b
///   .P`                              '¶,
///  .P                                  ¶,
/// .P          ]b           .d`          ¶,
/// d             ¶b       .d"             b
/// [               "_   .d"               ]
/// [                 ¶bd"                 ]
/// [                .d""_                 ]
/// [              .d"    "_,              ]
/// ¶            .d"       '"b             P
/// 'b          ]"            "           d`
///  'b                                  d`
///   'b,                              .d`
///     ¶_                            _P
///      '¶_,                      ._P`
///         "b_,                ._d"
///            ""¶____________P""
/// ```
///
pub const PURE_ASCII: [char; 16] = [
    ' ',  // 0000
    '.',  // 0001
    ',',  // 0010
    '_',  // 0011
    '\'', // 0100
    ']',  // 0101
    '/',  // 0110
    'd',  // 0111
    '`',  // 1000
    '\\', // 1001
    '[',  // 1010
    'b',  // 1011
    '"',  // 1100
    '¶', //  1101
    'P',  // 1110
    '#',  // 1111
];

/// Uses Braille characters to get clean, true-to-form shapes. It's not ASCII, but it looks great!
///
/// ```text
/// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣤⣼⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⣧⣤⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
/// ⠀⠀⠀⠀⠀⠀⠀⠀⣤⡟⠛⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠛⢻⣤⠀⠀⠀⠀⠀⠀⠀⠀
/// ⠀⠀⠀⠀⠀⢠⣼⠛⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠛⣧⡄⠀⠀⠀⠀⠀
/// ⠀⠀⠀⠀⣼⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⣧⠀⠀⠀⠀
/// ⠀⠀⢠⡟⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢻⡄⠀⠀
/// ⠀⢠⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡄⠀
/// ⢠⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣼⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡄
/// ⣼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣧⠀⠀⠀⠀⠀⠀⠀⢠⣼⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣧
/// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⣤⠀⠀⠀⢠⣼⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
/// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣧⣼⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
/// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣼⠛⠛⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
/// ⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣼⠛⠀⠀⠀⠀⠛⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
/// ⢻⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣼⠛⠀⠀⠀⠀⠀⠀⠀⠘⠛⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡟
/// ⠘⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠃
/// ⠀⠘⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⠃⠀
/// ⠀⠀⠘⣧⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣼⠃⠀⠀
/// ⠀⠀⠀⠀⢻⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⡟⠀⠀⠀⠀
/// ⠀⠀⠀⠀⠀⠘⢻⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣤⡟⠃⠀⠀⠀⠀⠀
/// ⠀⠀⠀⠀⠀⠀⠀⠀⠛⣧⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣤⣼⠛⠀⠀⠀⠀⠀⠀⠀⠀
/// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠛⢻⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⡟⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
/// ```
///
pub const BRAILLE: [char; 16] = [
    '\u{2800}', // 0000
    '\u{28a0}', // 0001
    '\u{2844}', // 0010
    '\u{28e4}', // 0011
    '\u{2818}', // 0100
    '\u{28b8}', // 0101
    '\u{285c}', // 0110
    '\u{28fc}', // 0111
    '\u{2803}', // 1000
    '\u{28a3}', // 1001
    '\u{2847}', // 1010
    '\u{28e7}', // 1011
    '\u{281b}', // 1100
    '\u{28bb}', // 1101
    '\u{285f}', // 1110
    '\u{28ff}', // 1111
];
