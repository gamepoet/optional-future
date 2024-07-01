use std::{
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::FusedFuture;
use pin_project_lite::pin_project;

/// A boxed [`OptionalFuture`].
pub type BoxOptionalFuture<T> = OptionalFuture<Pin<Box<dyn Future<Output = T> + Send + 'static>>>;

pin_project! {
  /// A future wrapped in an option. If the option is [`None`], then it can never resolve. This is
  /// useful when you want have a select! on a future that is only sometimes valid.
  #[derive(Debug)]
  #[must_use = "futures do nothing unless you `.await` or poll them"]
  pub struct OptionalFuture<Fut> {
    #[pin]
    inner: Option<Fut>,
  }
}

impl<Fut> OptionalFuture<Fut> {
    /// Replaces the the actual value in the option by the value in the parameter, returning the old
    /// value if present, leaving a [`Some`] in its place without deinitializing either one.
    pub fn replace(&mut self, value: Fut) -> Option<Fut> {
        mem::replace(&mut self.inner, Some(value))
    }

    /// Takes the value out of the option, leaving a [`None`] in its place.
    pub fn take(&mut self) -> Option<Fut> {
        mem::take(&mut self.inner)
    }

    /// Returns `true` if the option is a [`None`] value.
    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    /// Returns `true` if the option is a [`Some`] value.
    pub fn is_some(&self) -> bool {
        self.inner.is_some()
    }

    /// Gets the contained `Option<Fut>`` as an `Option<&Fut>`.
    pub fn as_ref(&self) -> Option<&Fut> {
        self.inner.as_ref()
    }
}

/// Returns [`None`].
impl<Fut> Default for OptionalFuture<Fut> {
    fn default() -> Self {
        Self { inner: None }
    }
}

/// Creates a new optional future from an Option.
pub fn optional_future<Fut>(option: Option<Fut>) -> OptionalFuture<Fut>
where
    Fut: Future,
{
    OptionalFuture { inner: option }
}

impl<F> Future for OptionalFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().inner.as_pin_mut() {
            Some(inner) => inner.poll(cx),
            None => Poll::Pending,
        }
    }
}

impl<F> FusedFuture for OptionalFuture<F>
where
    F: FusedFuture,
{
    fn is_terminated(&self) -> bool {
        match &self.inner {
            Some(inner) => inner.is_terminated(),
            None => true, // this can never change from Pending
        }
    }
}

impl<F> From<Option<F>> for OptionalFuture<F> {
    fn from(value: Option<F>) -> Self {
        Self { inner: value }
    }
}

impl<F> Clone for OptionalFuture<F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
