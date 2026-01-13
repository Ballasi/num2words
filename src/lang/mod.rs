mod lang;
mod en;
mod fr;
mod it;
mod uk;

pub use en::English;
pub use fr::French;
pub use it::Italian;
pub use uk::Ukrainian;

pub use lang::to_language;
pub use lang::Lang;
pub use lang::Language;
