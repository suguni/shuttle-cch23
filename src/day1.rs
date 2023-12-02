use std::ops::BitXor;
use std::str::FromStr;

use warp::{Filter, Rejection, Reply};
use warp::path::Peek;

pub fn day1_router() -> warp::filters::BoxedFilter<(impl Reply, )> {
    // warp::get()
    //     .and(warp::path!("1" / u32 / u32))
    //     .and_then(day1_task1)
    //     .boxed()

    warp::get()
        .and(warp::path("1"))
        .and(warp::path::peek())
        .and_then(day1_task2)
        .boxed()
}

async fn day1_task1(
    num1: u32,
    num2: u32,
) -> Result<impl Reply, Rejection> {
    Ok(num1.bitxor(num2).pow(3).to_string())
}

async fn day1_task2(
    peek: Peek,
) -> Result<impl Reply, Rejection> {
    let mut xored = 0_i64;

    for v in peek.segments().take(20) {
        if let Ok(n) = i64::from_str(v) {
            xored = xored.bitxor(n);
        } else {
            return Err(warp::reject::not_found())
        }
    }

    Ok(xored.pow(3).to_string())
}
