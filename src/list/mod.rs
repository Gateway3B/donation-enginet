use std::ops::ControlFlow;

use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct List {
    pub id: i32,
    pub user_id: String,
    pub budget: Budget,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Serialize)]
pub struct Budget {
    pub id: i32,
    pub list_id: i32,

    pub total_value: Decimal,
    pub donation_percent: Decimal,
    pub value_override: Option<Decimal>,

    pub temp_donation_value: Decimal,
}

#[derive(Deserialize, Serialize)]
pub struct Category {
    pub id: i32,
    pub list_id: i32,

    pub name: String,
    pub entries: Vec<Entry>,

    pub multiplier: Decimal,
    pub percent_override: Option<Decimal>,
    pub value_override: Option<Decimal>,

    pub enabled: bool,

    pub temp_donation_value: Decimal,
    pub temp_donation_percent: Decimal,

    pub temp_included: bool,
    pub temp_has_entry_overrides: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub id: i32,
    pub category_id: i32,

    pub ein: i32,

    pub multiplier: Decimal,
    pub percent_override: Option<Decimal>,
    pub value_override: Option<Decimal>,

    pub enabled: bool,

    pub temp_donation_value: Decimal,
    pub temp_donation_percent: Decimal,
}

impl List {
    pub fn is_list_valid(self) -> bool {
        let mut valid = true;

        if self.categories.is_empty() {
            return valid;
        }

        let mut categories_value_sum = Decimal::default();
        let mut categories_percent_sum = Decimal::default();
        let mut entry_count = 0;

        self.categories
            .iter()
            .filter(|category| category.temp_included)
            .filter(|category| !category.entries.is_empty())
            .try_for_each(|category| {
                if category.temp_donation_value == Decimal::ZERO
                    || category.temp_donation_percent == Decimal::ZERO
                {
                    valid = false;
                    return ControlFlow::Break(());
                }

                categories_value_sum += category.temp_donation_value;
                categories_percent_sum += category.temp_donation_percent;
                entry_count += category.entries.len();

                let mut entries_value_sum = Decimal::default();
                let mut entries_percent_sum = Decimal::default();

                category
                    .entries
                    .iter()
                    .filter(|entry| entry.enabled)
                    .try_for_each(|entry| {
                        if entry.temp_donation_value == Decimal::ZERO
                            || entry.temp_donation_percent == Decimal::ZERO
                        {
                            valid = false;
                            return ControlFlow::Break(());
                        }

                        entries_value_sum += entry.temp_donation_value;
                        entries_percent_sum += entry.temp_donation_percent;
                        ControlFlow::Continue(())
                    });

                let entries_value_overflow =
                    (entries_value_sum - category.temp_donation_value).abs() > Decimal::ONE;
                let entries_percent_overflow =
                    (entries_percent_sum - Decimal::ONE).abs() > Decimal::new(1, 2);

                if entries_value_overflow || entries_percent_overflow {
                    valid = false;
                    return ControlFlow::Break(());
                }

                ControlFlow::Continue(())
            });

        if !valid || entry_count == 0 {
            return valid;
        }

        let categories_value_overflow =
            (categories_value_sum - self.budget.temp_donation_value).abs() > Decimal::ONE;
        let categories_percent_overflow =
            (categories_percent_sum - Decimal::ONE).abs() > Decimal::new(1, 2);

        if categories_value_overflow || categories_percent_overflow {
            valid = false;
        }

        valid
    }

    pub fn process_list(&mut self) {
        self.process_budget();
        self.process_categories();
        self.process_entries();
    }

    fn process_budget(&mut self) {
        self.budget.temp_donation_value = self
            .budget
            .value_override
            .unwrap_or((self.budget.total_value * self.budget.donation_percent).round_dp(2));
    }

    fn process_categories(&mut self) {
        let mut multiplier_sum = Decimal::default();
        let mut value_override_sum = Decimal::default();
        let mut percent_override_sum = Decimal::default();

        let mut multiplier_categories = Vec::new();

        self.categories.iter_mut().for_each(|category| {
            category.temp_included = category.enabled && !category.entries.is_empty();

            if !category.temp_included {
                return;
            }

            if let Some(value_override) = category.value_override {
                category.temp_donation_value = value_override;
                category.temp_donation_percent =
                    (value_override / self.budget.temp_donation_value).round_dp(2);

                value_override_sum += value_override;
                percent_override_sum += category.temp_donation_percent;
            } else if let Some(percent_override) = category.percent_override {
                category.temp_donation_value =
                    (percent_override * self.budget.temp_donation_value).round_dp(2);
                category.temp_donation_percent = percent_override;

                value_override_sum += category.temp_donation_value;
                percent_override_sum += percent_override;
            } else {
                multiplier_sum += category.multiplier;
                multiplier_categories.push(category);
            }
        });

        let left_over_cash = self.budget.temp_donation_value - value_override_sum;
        let left_over_percent = Decimal::default() - percent_override_sum;

        multiplier_categories.iter_mut().for_each(|category| {
            let category_multiplier_adjustment = (category.multiplier / multiplier_sum).round_dp(2);

            category.temp_donation_value =
                (left_over_cash * category_multiplier_adjustment).round_dp(2);
            category.temp_donation_percent =
                (left_over_percent * category_multiplier_adjustment).round_dp(2);
        });
    }

    fn process_entries(&mut self) {
        self.categories
            .iter_mut()
            .for_each(|category| List::process_category_entries(category));
    }

    fn process_category_entries(category: &mut Category) {
        let mut multiplier_sum = Decimal::default();
        let mut value_override_sum = Decimal::default();
        let mut percent_override_sum = Decimal::default();

        let mut multiplier_entries = Vec::new();

        category
            .entries
            .iter_mut()
            .filter(|entry| entry.enabled && category.enabled)
            .for_each(|entry| {
                if let Some(value_override) = entry.value_override {
                    entry.temp_donation_value = value_override;
                    entry.temp_donation_percent =
                        (value_override / category.temp_donation_value).round_dp(2);

                    value_override_sum += value_override;
                    percent_override_sum += entry.temp_donation_percent;

                    category.temp_has_entry_overrides = true;
                } else if let Some(percent_override) = entry.percent_override {
                    entry.temp_donation_value =
                        (percent_override * category.temp_donation_value).round_dp(2);
                    entry.temp_donation_percent = percent_override;

                    value_override_sum += entry.temp_donation_value;
                    percent_override_sum += percent_override;

                    category.temp_has_entry_overrides = true;
                } else {
                    multiplier_sum += entry.multiplier;
                    multiplier_entries.push(entry);
                }
            });

        let left_over_cash = category.temp_donation_value - value_override_sum;
        let left_over_percent = Decimal::default() - percent_override_sum;

        multiplier_entries.iter_mut().for_each(|entry| {
            let entry_multiplier_adjustment = (entry.multiplier / multiplier_sum).round_dp(2);

            entry.temp_donation_value = (left_over_cash * entry_multiplier_adjustment).round_dp(2);
            entry.temp_donation_percent =
                (left_over_percent * entry_multiplier_adjustment).round_dp(2);
        });
    }
}

use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {
use sea_orm::*;

use crate::entity::budget::{
    ActiveModel as ActiveBudgetModel, Column as BudgetColumn, Entity as BudgetEntity,
    Model as BudgetModel,
};
use crate::entity::category::{
    ActiveModel as ActiveCategoryModel, Column as CategoryColumn, Entity as CategoryEntity,
    Model as CategoryModel,
};
use crate::entity::entry::{
    ActiveModel as ActiveEntryModel, Column as EntryColumn, Entity as EntryEntity,
    Model as EntryModel,
};
use crate::entity::list::{
    ActiveModel as ActiveListModel, Column as ListColumn, Entity as ListEntity, Model as ListModel,
};

impl From<(ListModel, Budget, Vec<Category>)> for List {
    fn from(value: (ListModel, Budget, Vec<Category>)) -> Self {
        Self {
            id: value.0.id,
            user_id: value.0.user_id,
            budget: value.1,
            categories: value.2,
        }
    }
}

impl From<&List> for ListModel {
    fn from(value: &List) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id.clone(),
        }
    }
}

impl From<BudgetModel> for Budget {
    fn from(value: BudgetModel) -> Self {
        Self {
            id: value.id,
            list_id: value.list_id,
            total_value: value.total_value,
            donation_percent: value.donation_percent,
            value_override: value.value_override,
            temp_donation_value: Decimal::ZERO,
        }
    }
}

impl From<&Budget> for BudgetModel {
    fn from(value: &Budget) -> Self {
        Self {
            id: value.id,
            list_id: value.list_id,
            total_value: value.total_value,
            donation_percent: value.donation_percent,
            value_override: value.value_override,
        }
    }
}

impl From<(CategoryModel, Vec<Entry>)> for Category {
    fn from(value: (CategoryModel, Vec<Entry>)) -> Self {
        Self {
            id: value.0.id,
            list_id: value.0.list_id,
            name: value.0.name,
            entries: value.1,
            multiplier: value.0.multiplier,
            percent_override: value.0.percent_override,
            value_override: value.0.value_override,
            enabled: value.0.enabled,
            temp_donation_value: Decimal::ZERO,
            temp_donation_percent: Decimal::ZERO,
            temp_included: false,
            temp_has_entry_overrides: false,
        }
    }
}

impl From<&Category> for CategoryModel {
    fn from(value: &Category) -> Self {
        Self {
            id: value.id,
            list_id: value.list_id,
            name: value.name.clone(),
            multiplier: value.multiplier,
            percent_override: value.percent_override,
            value_override: value.value_override,
            enabled: value.enabled,
        }
    }
}

impl From<EntryModel> for Entry {
    fn from(value: EntryModel) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            ein: value.ein,
            multiplier: value.multiplier,
            percent_override: value.percent_override,
            value_override: value.value_override,
            enabled: value.enabled,
            temp_donation_value: Decimal::ZERO,
            temp_donation_percent: Decimal::ZERO,
        }
    }
}

impl From<&Entry> for EntryModel {
    fn from(value: &Entry) -> Self {
        Self {
            id: value.id,
            category_id: value.category_id,
            ein: value.ein,
            multiplier: value.multiplier,
            percent_override: value.percent_override,
            value_override: value.value_override,
            enabled: value.enabled,
        }
    }
}

impl List {
    pub async fn from_user_id(db: &DatabaseConnection, user_id: String) -> Option<List> {
        let list_model: ListModel = ListEntity::find()
            .filter(ListColumn::UserId.eq(user_id))
            .one(db)
            .await
            .ok()??;
        let budget_model: BudgetModel = BudgetEntity::find()
            .filter(BudgetColumn::ListId.eq(list_model.id))
            .one(db)
            .await
            .ok()??;
        let category_models: Vec<CategoryModel> = CategoryEntity::find()
            .filter(CategoryColumn::ListId.eq(list_model.id))
            .all(db)
            .await
            .ok()?;
        let entry_models: Vec<Vec<EntryModel>> =
            category_models.load_many(EntryEntity, db).await.ok()?;

        let entries: Vec<Vec<Entry>> = entry_models
            .into_iter()
            .map(|entries| entries.into_iter().map(|entry| entry.into()).collect())
            .collect();
        let categories: Vec<Category> = category_models
            .into_iter()
            .zip(entries.into_iter())
            .map(|(category, entries)| (category, entries).into())
            .collect();
        let budget: Budget = budget_model.into();
        let list: List = (list_model, budget, categories).into();

        Some(list)
    }

    pub async fn init_list(db: &DatabaseConnection, user_id: String) -> Option<List> {
        let list = ActiveListModel {
            id: NotSet,
            user_id: Set(user_id.clone()),
        }
        .save(db)
        .await.ok()?;

        ActiveBudgetModel {
            id: NotSet,
            list_id: list.id.clone(),
            total_value: Set(Decimal::new(50_000, 0)),
            donation_percent: Set(Decimal::new(10, 2)),
            value_override: Set(None),
        }
        .save(db)
        .await.ok()?;

        List::from_user_id(db, user_id).await
    }
    // fn save(self) {
    //     let entry_models: Vec<Vec<EntryModel>> = self.categories.iter().map(|category| category.entries.iter().map(|entry| entry.into()).collect()).collect();
    //     let category_models: Vec<CategoryModel> = self.categories.iter().map(|category| category.into()).collect();
    //     let budget_model: BudgetModel = (&self.budget).into();
    //     let list_model: ListModel = (&self).into();

    //     let list_model: ActiveListModel = list_model.into();
    //     let budget_model: ActiveBudgetModel = budget_model.into();
    //     let category_models: Vec<ActiveCategoryModel> = category_models.into_iter().map(|category| category.into()).collect();
    //     let entry_models: Vec<Vec<ActiveEntryModel>> = entry_models.into_iter().map(|category| category.into_iter().map(|entry| entry.into()).collect()).collect();

    //     entry_models.

    // }
}
}}
