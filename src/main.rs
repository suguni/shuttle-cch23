use warp::Filter;
use warp::http::StatusCode;
use warp::Reply;

use crate::day1::day1_router;

mod day1;

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let hello = warp::get()
        .and(warp::path::end())
        .map(|| "Hello, world!");

    let error = warp::get()
        .and(warp::path!("-1" / "error"))
        .and(warp::path::end())
        .then(|| async { warp::reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR) });

    let route = hello
        .or(error)
        .or(day1_router())
        // .recover(|r| async move {
        //     println!("===>>>> {:?}", r);
        //     Ok::<WithStatus<_>, Rejection>(warp::reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
        // })
        ;

    Ok(route.boxed().into())
}
