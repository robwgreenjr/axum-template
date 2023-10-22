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

#[derive(Debug)]
pub struct ParameterQueryResult {
    pub filter_list: Vec<Vec<String>>,
    pub sort_list: HashMap<QuerySort, Vec<String>>,
    pub limit: u8,
}

impl ParameterQueryResult {
    fn build_query_result(query: Option<String>) -> ParameterQueryResult {
        let mut result = ParameterQueryResult {
            filter_list: vec![],
            sort_list: HashMap::new(),
            limit: 200,
        };

        let query_string = query.as_ref().unwrap();
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

        // TODO: Build sort list
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

        // TODO: Build filter list

        // TODO: Figure out how to handle errors properly for the application in general

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
    use crate::global::parameter_query_builder::{ParameterQueryResult, QuerySort};

    // TODO: Handle errors properly, will need to return correct error responses
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
}