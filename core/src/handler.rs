use crate::{async_trait, Depot, FlowCtrl, Request, Response};

/// `Handler` is used for handle [`Request`].
///
/// * `Handler` can be used as middleware to handle [`Request`].
///
/// # Example
///
/// ```
/// use salvo_core::prelude::*;
///
/// #[handler]
/// async fn middleware() {
/// }
///
/// #[tokio::main]
/// async fn main() {
///     Router::new().hoop(middleware);
/// }
/// ```
///
/// * `Handler` can be used as endpoint to handle [`Request`].
///
/// # Example
///
/// ```
/// # use salvo_core::prelude::*;
///
/// #[handler]
/// async fn middleware() {
/// }
///
/// #[tokio::main]
/// async fn main() {
///     Router::new().handle(middleware);
/// }
/// ```
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    #[doc(hidden)]
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
    #[doc(hidden)]
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    /// Handle http request.
    #[must_use = "handle future must be used"]
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl);
}

macro_rules! handler_tuple_impls {
    ($(
        $Tuple:tt {
            $(($idx:tt) -> $T:ident,)+
        }
    )+) => {$(
        #[async_trait::async_trait]
        impl<$($T,)+> Handler for ($($T,)+) where $($T: Handler,)+
        {
            async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
                $(
                    if !res.is_stamped() {
                        self.$idx.handle(req, depot, res, ctrl).await;
                    }
                )+
            }
        })+
    }
}
#[doc(hidden)]
macro_rules! __for_each_handler_tuple {
    ($callback:ident) => {
        $callback! {
            1 {
                (0) -> A,
            }
            2 {
                (0) -> A,
                (1) -> B,
            }
            3 {
                (0) -> A,
                (1) -> B,
                (2) -> C,
            }
            4 {
                (0) -> A,
                (1) -> B,
                (2) -> C,
                (3) -> D,
            }
            5 {
                (0) -> A,
                (1) -> B,
                (2) -> C,
                (3) -> D,
                (4) -> E,
            }
            6 {
                (0) -> A,
                (1) -> B,
                (2) -> C,
                (3) -> D,
                (4) -> E,
                (5) -> F,
            }
            7 {
                (0) -> A,
                (1) -> B,
                (2) -> C,
                (3) -> D,
                (4) -> E,
                (5) -> F,
                (6) -> G,
            }
            8 {
                (0) -> A,
                (1) -> B,
                (2) -> C,
                (3) -> D,
                (4) -> E,
                (5) -> F,
                (6) -> G,
                (7) -> H,
            }
        }
    };
}

__for_each_handler_tuple!(handler_tuple_impls);
