use crate::body::OptionReqBody;
use bytes::Bytes;
use http::{HeaderMap, Method};
use http_body_util::BodyExt;
use micro_http::protocol::{ParseError, RequestHeader};

pub trait FromRequest<'r> {
    type Output;
    async fn from_request(req: &'r RequestHeader, body: OptionReqBody) -> Result<Self::Output, ParseError>;
}

/// impl `FromRequest` for tuples
///
/// for example, it will impl Fn(A, B) like this:
///
/// ```no_run
/// # use micro_http::protocol::{HttpError, ParseError, RequestHeader};
/// # use micro_web::FromRequest;
///
/// impl<'r, A, B> FromRequest<'r> for (A, B)
/// where
///     A: FromRequest<'r>,
///     B: FromRequest<'r>,
/// {
///     type Output = (A::Output, B::Output);
///
///     async fn from_request(req: &'r RequestHeader, body: OptionReqBody) -> Result<Self::Output, ParseError> {
///         let a = A::from_request(req, body.clone()).await?;
///         let b = B::from_request(req, body.clone()).await?;
///         Ok((a, b))
///     }
/// }
/// ```
macro_rules! impl_from_request_for_fn ({ $($param:ident)* } => {
    impl<'r, $($param,)*> FromRequest<'r> for ($($param,)*)
    where
        $($param: FromRequest<'r>,)*
    {
        type Output = ($($param::Output,)*);
        async fn from_request(req: &'r RequestHeader, body: OptionReqBody) -> Result<Self::Output, ParseError> {
            Ok(($($param::from_request(req, body.clone()).await?,)*))
        }
    }
});

impl_from_request_for_fn! {}
impl_from_request_for_fn! {A}
impl_from_request_for_fn! {A B}
impl_from_request_for_fn! {A B C}
impl_from_request_for_fn! { A B C D }
impl_from_request_for_fn! { A B C D E }
impl_from_request_for_fn! { A B C D E F }
impl_from_request_for_fn! { A B C D E F G }
impl_from_request_for_fn! { A B C D E F G H }
impl_from_request_for_fn! { A B C D E F G H I }
impl_from_request_for_fn! { A B C D E F G H I J }
impl_from_request_for_fn! { A B C D E F G H I J K }
impl_from_request_for_fn! { A B C D E F G H I J K L }

impl<'r> FromRequest<'r> for Method {
    type Output = Method;

    async fn from_request(req: &'r RequestHeader, _body: OptionReqBody) -> Result<Self::Output, ParseError> {
        Ok(req.method().clone())
    }
}
impl<'r> FromRequest<'r> for &RequestHeader {
    type Output = &'r RequestHeader;

    async fn from_request(req: &'r RequestHeader, _body: OptionReqBody) -> Result<Self::Output, ParseError> {
        Ok(req)
    }
}

impl<'r> FromRequest<'r> for &HeaderMap {
    type Output = &'r HeaderMap;

    async fn from_request(req: &'r RequestHeader, _body: OptionReqBody) -> Result<Self::Output, ParseError> {
        Ok(req.headers())
    }
}

impl<'r> FromRequest<'r> for Bytes {
    type Output = Bytes;

    async fn from_request(_req: &'r RequestHeader, body: OptionReqBody) -> Result<Self::Output, ParseError> {
        body.apply(|b| async { b.collect().await.map(|c| c.to_bytes()) }).await
    }
}
