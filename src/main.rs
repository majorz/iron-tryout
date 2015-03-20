extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::headers::UserAgent;

fn main() {
   fn hello_world(request: &mut Request) -> IronResult<Response> {
      let user_agent = request.headers.get::<UserAgent>().unwrap();
      let content = format!("{:?}", user_agent);
      Ok(Response::with((status::Ok, content)))
   }

   Iron::new(hello_world).http("localhost:8888").unwrap();
   println!("On 8888");
}
