mod lang;
mod en;
mod fr;
mod uk;

mod nl;

mod fy;

pub use en::English;
pub use fr::French;
pub use uk::Ukrainian;
pub use nl::Dutch;
pub use fy::Frisian;

pub use lang::to_language;
pub use lang::Lang;
pub use lang::Language;
