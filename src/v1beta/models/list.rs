use deranged::{OptionRangedU16, RangedU16};
use reqwest::{Method, Url};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Default, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    /// The maximum number of `Model`s to return (per page).
    ///
    /// If unspecified, 50 models will be returned per page. This method returns at most 1000 models per page, even if you pass a larger `page_size`.
    #[builder(default)]
    #[serde(skip_serializing_if = "OptionRangedU16::is_none")]
    page_size: OptionRangedU16<0, 1000>,

    /// A page token, received from a previous list call.
    ///
    /// Provide the `page_token` returned by one request as an argument to the next request to retrieve the next page.
    ///
    /// When paginating, all other parameters provided to list must match the call that provided the page token.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

#[derive(Debug, TypedBuilder)]
pub struct Request {
    base: Url,

    query_params: QueryParams,
}

#[derive(Debug, thiserror::Error)]
pub enum IntoRequestError {
    #[error("{url} cannot be a base, so constructing the full URL for the request is impossible")]
    CannotBeABase { url: Url },
}

impl TryFrom<Request> for reqwest::Request {
    type Error = IntoRequestError;

    fn try_from(
        Request {
            mut base,

            query_params,
        }: Request,
    ) -> Result<Self, Self::Error> {
        let path_segments = base
            .path_segments_mut()
            .or(IntoRequestError::CannotBeABase { url: base })?;

        path_segments.push(crate::v1beta::PATH);

        let url = base;

        let method = Method::GET;

        Ok(reqwest::Request::new(method, url))
    }
}
