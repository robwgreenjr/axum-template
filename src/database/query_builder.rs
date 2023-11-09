use http::StatusCode;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, IdenStatic, Iterable, QueryFilter as QF, QueryOrder, QuerySelect};

use crate::global::error_handling::ErrorDetails;
use crate::global::parameter_query_builder::{ParameterQueryResult, QueryFilter, QuerySort};

pub struct QueryBuilder;

impl QueryBuilder {
    pub async fn get_list<E: EntityTrait>(
        db: &DatabaseConnection,
        query_result: ParameterQueryResult,
    ) -> Result<Vec<<E as EntityTrait>::Model>, Vec<ErrorDetails>>
    {
        let mut base_query = E::find()
            .limit(u64::from(query_result.limit));

        // TODO: Move sort logic to function
        for sort in query_result.sort_list {
            match sort.0 {
                QuerySort::ASC => {
                    for column in E::Column::iter() {
                        let column_name: &str = &column.as_str();

                        for item in &sort.1 {
                            let deref_item: String = item.into();

                            if deref_item == column_name {
                                base_query = base_query.clone().order_by_asc(column);

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
                                base_query = base_query.clone().order_by_desc(column);

                                break;
                            }
                        }
                    }
                }
            }
        }

        // TODO: Build filter
        for filterList in query_result.filter_list {
            let mut conditions = Condition::all();
            for filter in filterList.filter_list {
                for column in E::Column::iter() {
                    let column_name: &str = &column.as_str();

                    if filter.property != column_name.to_string() {
                        continue;
                    }

                    match filter.filter {
                        QueryFilter::GT => {
                            match parse_number(&filter.value) {
                                NumberType::Integer(value) => {
                                    conditions = conditions.clone().add(E::Column::gt(&column, value));
                                }
                                NumberType::Float(value) => {
                                    conditions = conditions.clone().add(E::Column::gt(&column, value));
                                }
                                NumberType::Invalid => {
                                    conditions = conditions.clone().add(E::Column::gt(&column, filter.value));
                                }
                            }
                        }
                        QueryFilter::GTE => {
                            match parse_number(&filter.value) {
                                NumberType::Integer(value) => {
                                    conditions = conditions.clone().add(E::Column::gte(&column, value));
                                }
                                NumberType::Float(value) => {
                                    conditions = conditions.clone().add(E::Column::gte(&column, value));
                                }
                                NumberType::Invalid => {
                                    conditions = conditions.clone().add(E::Column::gte(&column, filter.value));
                                }
                            }
                        }
                        QueryFilter::LT => {
                            match parse_number(&filter.value) {
                                NumberType::Integer(value) => {
                                    conditions = conditions.clone().add(E::Column::lt(&column, value));
                                }
                                NumberType::Float(value) => {
                                    conditions = conditions.clone().add(E::Column::lt(&column, value));
                                }
                                NumberType::Invalid => {
                                    conditions = conditions.clone().add(E::Column::lt(&column, filter.value));
                                }
                            }
                        }
                        QueryFilter::LTE => {
                            match parse_number(&filter.value) {
                                NumberType::Integer(value) => {
                                    conditions = conditions.clone().add(E::Column::lte(&column, value));
                                }
                                NumberType::Float(value) => {
                                    conditions = conditions.clone().add(E::Column::lte(&column, value));
                                }
                                NumberType::Invalid => {
                                    conditions = conditions.clone().add(E::Column::lte(&column, filter.value));
                                }
                            }
                        }
                        QueryFilter::EQ => {
                            match parse_number(&filter.value) {
                                NumberType::Integer(value) => {
                                    conditions = conditions.clone().add(E::Column::eq(&column, value));
                                }
                                NumberType::Float(value) => {
                                    conditions = conditions.clone().add(E::Column::eq(&column, value));
                                }
                                NumberType::Invalid => {
                                    conditions = conditions.clone().add(E::Column::eq(&column, filter.value));
                                }
                            }
                        }
                        QueryFilter::NE => {
                            match parse_number(&filter.value) {
                                NumberType::Integer(value) => {
                                    conditions = conditions.clone().add(E::Column::ne(&column, value));
                                }
                                NumberType::Float(value) => {
                                    conditions = conditions.clone().add(E::Column::ne(&column, value));
                                }
                                NumberType::Invalid => {
                                    conditions = conditions.clone().add(E::Column::ne(&column, filter.value));
                                }
                            }
                        }
                        QueryFilter::LIKE => {
                            conditions = conditions.clone().add(E::Column::contains(&column, filter.value));
                        }
                        QueryFilter::CURSOR => {
                            match parse_number(&filter.value) {
                                NumberType::Integer(value) => {
                                    conditions = conditions.clone().add(E::Column::gte(&column, value));
                                }
                                NumberType::Float(value) => {
                                    conditions = conditions.clone().add(E::Column::gte(&column, value));
                                }
                                NumberType::Invalid => {
                                    conditions = conditions.clone().add(E::Column::gte(&column, filter.value));
                                }
                            }
                        }
                    }

                    break;
                }
            }

            base_query = base_query.clone().filter(conditions);
        }

        match base_query
            .all(db)
            .await {
            Ok(result) => {
                Ok(result)
            }
            Err(_error) => {
                Err(vec![ErrorDetails {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "".to_string(),
                }])
            }
        }
    }
}

enum NumberType {
    Integer(i32),
    Float(f32),
    Invalid,
}

fn parse_number(input: &String) -> NumberType {
    if let Ok(integer) = input.parse() {
        return NumberType::Integer(integer);
    } else if let Ok(float) = input.parse() {
        return NumberType::Float(float);
    }

    NumberType::Invalid
}
