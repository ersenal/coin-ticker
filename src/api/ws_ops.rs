use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize)]
pub struct BinanceOpPayload<const METHOD: &'static str, P = ()> {
    id: u32,
    method: &'static str,
    params: P,
}

pub trait BinanceOp<const ID: u32, const METHOD: &'static str, P = ()> {
    #[allow(clippy::new_ret_no_self)]
    fn new(params: P) -> BinanceOpPayload<METHOD, P> {
        BinanceOpPayload {
            id: ID,
            method: METHOD,
            params,
        }
    }
}

impl<const METHOD: &'static str, P: serde::Serialize> From<BinanceOpPayload<METHOD, P>>
    for Message
{
    fn from(p: BinanceOpPayload<METHOD, P>) -> Message {
        serde_json::to_string(&p).unwrap().into()
    }
}

macro_rules! op {
    ($i: ident, $op: literal, $code: literal) => {
        op!($i, $op, $code, ());
    };
    ($i: ident, $op: literal, $code: literal, $params: ty) => {
        pub type $i = BinanceOpPayload<$op>;
        impl BinanceOp<$code, $op, $params> for $i {}
    };
}

op!(Subscribe, "SUBSCRIBE", 1, Vec<String>);
