extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
   fn hello_world(request: &mut Request) -> IronResult<Response> {
      let content = format!("{:?}", request);
      Ok(Response::with((status::Ok, content)))
   }

   Iron::new(hello_world).http("localhost:8888").unwrap();
   println!("On 8888");
}
