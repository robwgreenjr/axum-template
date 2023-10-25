use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, IdenStatic, Iterable, QueryFilter, QueryOrder, QuerySelect};

use crate::global::parameter_query_builder::{ParameterQueryResult, QueryFilter as QF, QuerySort};

pub struct QueryBuilder;

impl QueryBuilder {
    pub async fn get_list<E: EntityTrait>(
        db: &DatabaseConnection,
        query_result: ParameterQueryResult,
    ) -> Vec<<E as EntityTrait>::Model>
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
                        QF::GT => {
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
                        QF::GTE => {
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
                        QF::LT => {
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
                        QF::LTE => {
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
                        QF::EQ => {
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
                        QF::NE => {
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
                        QF::LIKE => {
                            conditions = conditions.clone().add(E::Column::contains(&column, filter.value));
                        }
                        QF::CURSOR => {
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

        base_query
            .all(db)
            .await
            .expect("Cannot find entity")
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