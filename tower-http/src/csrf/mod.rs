use std::{
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{ready, Context, Poll},
};

use http::{HeaderName, Response};
use pin_project_lite::pin_project;
use tower::Layer;

#[derive(Clone, Default)]
pub struct CrossSiteRequestForgeryLayer {
    headers: Arc<[HeaderName]>,
}

impl CrossSiteRequestForgeryLayer {
    fn new() -> Self {
        Self::default()
    }
    fn assert_headers<T>(&self, target: &mut Response<T>) {}
}

#[derive(Clone)]
pub struct CrossSiteRequestForgery<S> {
    inner: S,
    layer: CrossSiteRequestForgeryLayer,
}

impl<S> Layer<S> for CrossSiteRequestForgeryLayer {
    type Service = CrossSiteRequestForgery<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CrossSiteRequestForgery {
            inner,
            layer: self.clone(),
        }
    }
}

pin_project! {
    /// Response future for [`CrossSiteRequestForgery`].
    pub struct ResponseFuture<F> {
        #[pin]
        future: F,
        headers: Arc<[HeaderName]>,
    }
}

impl<F, ResBody, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let mut res = ready!(this.future.poll(cx)?);

        this.headers.iter().for_each(|header| {});

        // this.mode.apply(this.header_name, &mut res, &mut *this.make);

        Poll::Ready(Ok(res))
    }
}
