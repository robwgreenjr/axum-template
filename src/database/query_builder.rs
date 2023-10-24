use sea_orm::{DatabaseConnection, EntityTrait, IdenStatic, Iterable, QueryOrder, QuerySelect};

use crate::global::parameter_query_builder::{ParameterQueryResult, QuerySort};

pub struct QueryBuilder;

impl QueryBuilder {
    pub async fn get_list<E: EntityTrait>(
        db: &DatabaseConnection,
        query_result: ParameterQueryResult,
    ) -> Vec<<E as EntityTrait>::Model>
    {
        let mut base_query = E::find()
            .limit(u64::from(query_result.limit));

        for sort in query_result.sort_list {
            match sort.0 {
                QuerySort::ASC => {
                    for column in E::Column::iter() {
                        let column_name: &str = &column.as_str();

                        for item in &sort.1 {
                            let deref_item: String = item.into();

                            if deref_item == column_name {
                                let mut sorted_query_clone = base_query.clone();
                                sorted_query_clone = sorted_query_clone.clone().order_by_asc(column);
                                base_query = sorted_query_clone;

                                break;
                            }
                        }
                    }
                }
                QuerySort::DESC => {
                    for column in E::Column::iter() {
                        let column_name: &str = &column.as_str();

                        for item in &sort.1 {
                            let deref_item: String = item.into();

                            if deref_item == column_name {
                                let mut sorted_query_clone = base_query.clone();
                                sorted_query_clone = sorted_query_clone.clone().order_by_desc(column);
                                base_query = sorted_query_clone;

                                break;
                            }
                        }
                    }
                }
            }
        }


        // TODO: Build filter

        base_query
            .all(db)
            .await
            .expect("Cannot find entity")
    }
}
