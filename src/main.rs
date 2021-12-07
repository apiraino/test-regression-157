use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {}

pub async fn do_it() -> Result<Box<dyn warp::Reply>, Infallible> {
    Ok(Box::new(warp::reply::json(&User {})))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // to further increase compile time, add new routers here
    let r = warp::get().and(warp::path!("employees")).and_then(do_it);
    let r1 = warp::get().and(warp::path!("employees1")).and_then(do_it);
    let r2 = warp::get().and(warp::path!("employees2")).and_then(do_it);
    let r3 = warp::get().and(warp::path!("employees3")).and_then(do_it);
    let r4 = warp::get().and(warp::path!("employees4")).and_then(do_it);
    let r5 = warp::get().and(warp::path!("employees5")).and_then(do_it);
    let r6 = warp::get().and(warp::path!("employees6")).and_then(do_it);
    let r7 = warp::get().and(warp::path!("employees7")).and_then(do_it);
    let r8 = warp::get().and(warp::path!("employees8")).and_then(do_it);
    let r9 = warp::get().and(warp::path!("employees9")).and_then(do_it);
    let r10 = warp::get().and(warp::path!("employees10")).and_then(do_it);
    let r11 = warp::get().and(warp::path!("employees11")).and_then(do_it);
    let r12 = warp::get().and(warp::path!("employees12")).and_then(do_it);
    let r13 = warp::get().and(warp::path!("employees13")).and_then(do_it);
    
    let r14 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r15 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r16 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r17 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r18 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r19 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r20 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r21 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r22 = warp::get().and(warp::path!("employees14")).and_then(do_it);
    let r23 = warp::get().and(warp::path!("employees14")).and_then(do_it);

    let routes = r
        .or(r)
        .or(r1)
        .or(r2)
        .or(r3)
        .or(r4)
        .or(r5)
        .or(r6)
        .or(r7)
        .or(r8)
        .or(r9)
        .or(r10)
        .or(r11)
        .or(r12)
        .or(r13)

        .or(r14)
        .or(r15)
        .or(r16)
        .or(r17)
        .or(r18)
        .or(r19)
        .or(r20)
        .or(r21)
        .or(r22)
        .or(r23)

        .with(warp::log("test-157"));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
