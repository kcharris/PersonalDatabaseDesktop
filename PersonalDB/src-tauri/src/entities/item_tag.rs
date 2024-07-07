use sea_orm::entity::prelude::*;
use crate::entities::item_tag::serde::Serialize;
use crate::entities::item_tag::serde::Deserialize;
use serde;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "item_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub item_id: i32,
    #[sea_orm(primary_key)]
    pub tag_id: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {
    
}