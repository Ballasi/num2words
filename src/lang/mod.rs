mod lang;
mod en;
mod fr;
mod tr;
mod uk;

pub use en::English;
pub use fr::French;
pub use tr::Turkish;
pub use uk::Ukrainian;

pub use lang::to_language;
pub use lang::Lang;
pub use lang::Language;
