//! 存放一些为了方便开发的脚手架
//! faq for TOMATOFAQ, not for frequently asked question!

/// python风格的print
macro_rules! print {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}, ", format_args!($($arg)*)));
}