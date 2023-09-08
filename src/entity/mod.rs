pub mod budget;
pub mod category;
pub mod color;
pub mod default_category;
pub mod entry;
pub mod list;

use sea_orm::prelude::Decimal;

pub trait DonationValue {
    fn donation_value(&self) -> Decimal;
}
