use axum::http::{Request, Response};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::Service;

#[derive(Clone)]
pub struct MyMiddleware<S> {
    pub inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for MyMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        println!("`MyMiddleware` called!");
        // println!("{:?}", req.headers());
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let res: Response<ResBody> = inner.call(req).await?;

            // println!("`MyMiddleware` received the response");

            Ok(res)
        })
    }
}