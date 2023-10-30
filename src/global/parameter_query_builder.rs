use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use http::request::Parts;

#[derive(Debug)]
pub struct ParameterQueryBuilder(pub ParameterQueryResult);

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum QuerySort {
    ASC,
    DESC,
}

impl FromStr for QuerySort {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ASC" => Ok(QuerySort::ASC),
            "DESC" => Ok(QuerySort::DESC),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum QueryFilter {
    GT,
    GTE,
    LT,
    LTE,
    EQ,
    NE,
    LIKE,
    CURSOR,
}

impl FromStr for QueryFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GT" => Ok(QueryFilter::GT),
            "GTE" => Ok(QueryFilter::GTE),
            "LT" => Ok(QueryFilter::LT),
            "LTE" => Ok(QueryFilter::LTE),
            "EQ" => Ok(QueryFilter::EQ),
            "NE" => Ok(QueryFilter::NE),
            "LIKE" => Ok(QueryFilter::LIKE),
            "CURSOR" => Ok(QueryFilter::CURSOR),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum QueryOperator {
    AND,
    OR,
}

impl FromStr for QueryOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AND" => Ok(QueryOperator::AND),
            "OR" => Ok(QueryOperator::OR),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct ParameterQueryResult {
    pub filter_list: Vec<ColumnFilterList>,
    pub sort_list: HashMap<QuerySort, Vec<String>>,
    pub limit: u8,
}

#[derive(Debug, PartialEq)]
pub struct ColumnFilter {
    pub operator: QueryOperator,
    pub filter: QueryFilter,
    pub property: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct ColumnFilterList {
    pub operator: QueryOperator,
    pub filter_list: Vec<ColumnFilter>,
}

impl ParameterQueryResult {
    fn build_query_result(query: Option<String>) -> ParameterQueryResult {
        // TODO: Figure out how to handle errors properly for the application in general
        let mut result = ParameterQueryResult {
            filter_list: vec![],
            sort_list: HashMap::new(),
            limit: 200,
        };

        let query_string;
        let has_query_string = query.as_ref();
        match has_query_string {
            None => {
                return result;
            }
            Some(value) => {
                query_string = value;
            }
        }

        let possible_params = query_string.split("&");

        // TODO: Move limit logic to function (create a builder like pattern for ParameterQueryResult?)
        let limit: Vec<_> = possible_params
            .clone()
            .filter(|param| {
                let is_limit_valid: Vec<_> = param.split("limit=").collect();
                param.contains("limit=") && param.split("limit=").count() == 2 && is_limit_valid[is_limit_valid.len() - 1].len() <= 3
            })
            .collect();
        if limit.len() > 0 {
            result.limit = limit[0].split("limit=").collect::<Vec<_>>()[1].parse().unwrap()
        }

        // TODO: Move sort logic to function (create a builder like pattern for ParameterQueryResult?)
        let sort_by: Vec<_> = possible_params
            .clone()
            .filter(|param| {
                param.contains("sort_by=") && param.split("sort_by=").count() == 2
            })
            .collect();

        if sort_by.is_empty() {
            let sort_fields = vec!["id".to_string()];
            let mut mapping = HashMap::new();
            mapping.insert(QuerySort::ASC, sort_fields);

            result.sort_list = mapping
        } else {
            let sorts = sort_by
                .first()
                .expect("Error finding first sort_by.")
                .split("sort_by=")
                .last()
                .unwrap();

            let sorts_seperated: Vec<_> = sorts
                .split("),")
                .map(|s| {
                    if s.ends_with(")") {
                        s.to_string()
                    } else {
                        format!("{})", s)
                    }
                })
                .collect();

            let mut mapping = HashMap::new();
            for sort in &sorts_seperated {
                let query_sort = sort.split("(").min().unwrap();

                let mut query_sort_value = QuerySort::ASC;
                match QuerySort::from_str(query_sort) {
                    Ok(parsed_sort) => match parsed_sort {
                        QuerySort::ASC => {
                            // Do nothing, use default
                        }
                        QuerySort::DESC => {
                            query_sort_value = QuerySort::DESC
                        }
                    },
                    Err(_) => {
                        // TODO: handle errors properly
                        println!("Query sort is neither ASC nor DESC");
                    }
                }

                let mut sort_list = sort
                    .split("(")
                    .last()
                    .unwrap()
                    .to_string();
                sort_list.pop();

                mapping.insert(query_sort_value, sort_list.split(",").map(String::from).collect());
            }

            result.sort_list = mapping
        }

        // TODO: Move filter logic to function (create a builder like pattern for ParameterQueryResult?)
        let filters: Vec<_> = possible_params
            .clone()
            .filter(|param| {
                !param.contains("sort_by=") && !param.contains("limit=")
            })
            .collect();

        for filter in &filters {
            let mut column_filter_list_operator = QueryOperator::AND;
            let mut cleaned_filter;
            let check_filter_list_operator = filter.clone();

            match check_filter_list_operator {
                s if s.starts_with(&format!("[{:?}]", QueryOperator::OR).to_lowercase()) => {
                    column_filter_list_operator = QueryOperator::OR;

                    cleaned_filter = check_filter_list_operator.trim_start_matches(&format!("[{:?}]", QueryOperator::OR).to_lowercase()).to_string();
                }
                s if s.starts_with(&format!("[{:?}]", QueryOperator::AND).to_lowercase()) => {
                    cleaned_filter = check_filter_list_operator.trim_start_matches(&format!("[{:?}]", QueryOperator::AND).to_lowercase()).to_string();
                }
                s if s.starts_with(&format!("[{:?}]", QueryOperator::OR)) => {
                    column_filter_list_operator = QueryOperator::OR;

                    cleaned_filter = check_filter_list_operator.trim_start_matches(&format!("[{:?}]", QueryOperator::OR)).to_string();
                }
                s if s.starts_with(&format!("[{:?}]", QueryOperator::AND)) => {
                    cleaned_filter = check_filter_list_operator.trim_start_matches(&format!("[{:?}]", QueryOperator::AND)).to_string();
                }
                _ => {
                    cleaned_filter = check_filter_list_operator.to_string();
                }
            }

            let mut column_filter_list = ColumnFilterList {
                operator: column_filter_list_operator,
                filter_list: vec![],
            };

            let mut operator = QueryOperator::AND;
            let mut query_filter = QueryFilter::EQ;
            let filter_variables: Vec<_> = cleaned_filter.split("=").collect();
            let value = filter_variables.last().unwrap().to_string();
            let mut property = filter_variables.first().unwrap().to_string();

            if property.is_empty() || value.is_empty() {
                continue;
            }

            let temp_property = property.clone();
            let mut find_operator_or_filters: Vec<_> = temp_property.split("[").collect();
            if let Some(first_filter) = find_operator_or_filters.first() {
                property = first_filter.to_string();
            }

            find_operator_or_filters.remove(0);
            for (index, operator_or_filter) in find_operator_or_filters.iter().enumerate() {
                let modified_operator_or_filter = operator_or_filter.to_string().replace("]", "");

                if index == 0 {
                    match QueryOperator::from_str(&*modified_operator_or_filter) {
                        Ok(parsed_operator) => match parsed_operator {
                            QueryOperator::AND => {
                                // Do nothing, use default
                                continue;
                            }
                            QueryOperator::OR => {
                                operator = QueryOperator::OR;
                                continue;
                            }
                        },
                        Err(_) => {
                            // TODO: handle errors properly
                            println!("Query operator is neither AND nor OR");
                        }
                    }
                }

                match QueryFilter::from_str(&*modified_operator_or_filter) {
                    Ok(parsed_filter) => match parsed_filter {
                        QueryFilter::GT => {
                            query_filter = QueryFilter::GT;
                        }
                        QueryFilter::GTE => {
                            query_filter = QueryFilter::GTE;
                        }
                        QueryFilter::LT => {
                            query_filter = QueryFilter::LT;
                        }
                        QueryFilter::LTE => {
                            query_filter = QueryFilter::LTE;
                        }
                        QueryFilter::EQ => {
                            // Do nothing, use default
                        }
                        QueryFilter::NE => {
                            query_filter = QueryFilter::NE;
                        }
                        QueryFilter::LIKE => {
                            query_filter = QueryFilter::LIKE;
                        }
                        QueryFilter::CURSOR => {
                            query_filter = QueryFilter::CURSOR;
                        }
                    },
                    Err(_) => {
                        // TODO: handle errors properly
                        println!("Query filter is neither GT, GTE, LT, LTE, EQ, NE, LIKE, nor CURSOR");
                    }
                }
            }

            let column_filter = ColumnFilter {
                operator,
                filter: query_filter,
                property,
                value,
            };
            column_filter_list.filter_list.push(column_filter);

            result.filter_list.push(column_filter_list);
        }

        result
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ParameterQueryBuilder
    where
        S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query()
                         .map(|query| query.to_owned());

        Ok(Self(ParameterQueryResult::build_query_result(query)))
    }
}

#[cfg(test)]
mod tests {
    use crate::global::parameter_query_builder::{ColumnFilter, ColumnFilterList, ParameterQueryResult, QueryFilter, QueryOperator, QuerySort};

// TODO: Handle errors properly, will need to return correct error responses

    /// Limits
    #[test]
    fn given_limit_should_return_correct_limit() {
        let result = ParameterQueryResult::build_query_result(Some("limit=35".parse().unwrap()));

        assert_eq!(result.limit, 35);
    }

    #[test]
    fn given_limit_along_with_other_params_should_return_correct_limit() {
        let result = ParameterQueryResult::build_query_result(Some("&test=testing&limit=155&testing=test".parse().unwrap()));

        assert_eq!(result.limit, 155);
    }

    /// Sorts
    #[test]
    fn given_no_sort_should_return_default() {
        let result = ParameterQueryResult::build_query_result(Some("".parse().unwrap()));
        let sort_fields = vec!["id".to_string()];

        assert_eq!(result.sort_list.get(&QuerySort::ASC), Some(&sort_fields));
    }

    #[test]
    fn given_single_sort_should_return_correct_sort() {
        let result = ParameterQueryResult::build_query_result(Some("sort_by=asc(first_field)".parse().unwrap()));
        let sort_fields = vec!["first_field".to_string()];

        assert_eq!(result.sort_list.get(&QuerySort::ASC), Some(&sort_fields));
    }

    #[test]
    fn given_double_sort_should_return_correct_sorts() {
        let result = ParameterQueryResult::build_query_result(Some("sort_by=asc(first_field),desc(second_field)".parse().unwrap()));
        let asc_sort_fields = vec!["first_field".to_string()];
        let desc_sort_fields = vec!["second_field".to_string()];

        assert_eq!(result.sort_list.get(&QuerySort::ASC), Some(&asc_sort_fields));
        assert_eq!(result.sort_list.get(&QuerySort::DESC), Some(&desc_sort_fields));
    }

    /// Filters
    #[test]
    fn given_no_filter_should_return_empty_filter() {
        let result = ParameterQueryResult::build_query_result(Some("".parse().unwrap()));
        let expected: Vec<ColumnFilterList> = vec![];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_single_filter_with_no_filter_or_operator_value_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::EQ,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_multi_filter_with_no_filter_or_operator_value_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name=value&second_field=value_two&third_field=value_three".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::EQ,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };

        let column_filter_two = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::EQ,
            property: "second_field".to_string(),
            value: "value_two".to_string(),
        };
        let column_filter_list_two = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter_two],
        };

        let column_filter_three = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::EQ,
            property: "third_field".to_string(),
            value: "value_three".to_string(),
        };
        let column_filter_list_three = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter_three],
        };

        let expected: Vec<ColumnFilterList> = vec![
            column_filter_list,
            column_filter_list_two,
            column_filter_list_three,
        ];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_greater_than_filter_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[gt]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::GT,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_greater_than_or_equal_to_filter_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[gte]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::GTE,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_less_than_or_equal_to_filter_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[lte]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::LTE,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_less_than_filter_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[lt]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::LT,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_equal_to_filter_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[eq]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::EQ,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_not_equal_to_filter_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[ne]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::NE,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_cursor_to_filter_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[cursor]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::CURSOR,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_filter_and_operator_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[or][gte]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::OR,
            filter: QueryFilter::GTE,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_filter_and_operator_with_invalid_order_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name[gte][or]=value".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::GTE,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list];

        assert_eq!(result.filter_list, expected);
    }

    #[test]
    fn given_or_operator_for_list_should_return_correct_filter() {
        let result = ParameterQueryResult::build_query_result(Some("field_name=value&[or]another_field=value_two".parse().unwrap()));
        let column_filter = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::EQ,
            property: "field_name".to_string(),
            value: "value".to_string(),
        };
        let column_filter_list = ColumnFilterList {
            operator: QueryOperator::AND,
            filter_list: vec![column_filter],
        };

        let column_filter_two = ColumnFilter {
            operator: QueryOperator::AND,
            filter: QueryFilter::EQ,
            property: "another_field".to_string(),
            value: "value_two".to_string(),
        };
        let column_filter_list_two = ColumnFilterList {
            operator: QueryOperator::OR,
            filter_list: vec![column_filter_two],
        };
        let expected: Vec<ColumnFilterList> = vec![column_filter_list, column_filter_list_two];

        assert_eq!(result.filter_list, expected);
    }
}