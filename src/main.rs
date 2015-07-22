extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate handlebars_iron;
extern crate urlencoded;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use staticfile::Static;
use mount::Mount;
use handlebars_iron::{Template, HandlebarsEngine};
use urlencoded::UrlEncodedBody;
use rustc_serialize::json::{ToJson};

use std::path::Path;
use std::collections::{BTreeMap};

extern crate sendmail;
use sendmail::email;

fn main() {

    let mut router = Router::new();

    //Routes
    fn index(req: &mut Request) -> IronResult<Response> {
        //Template Data
        let mut res = Response::new();
        let mut data = BTreeMap::new();
        data.insert("title".to_string(), "Index".to_json());

        //Template Page
        res.set_mut(Template::new("index", data)).set_mut(status::Ok);
        Ok(res)
    };
    fn contact(req: &mut Request) -> IronResult<Response> {
        //Template Data
        let mut res = Response::new();
        let mut data = BTreeMap::new();
        data.insert("title".to_string(), "Contact".to_json());

        //Template page
        res.set_mut(Template::new("contact", data)).set_mut(status::Ok);
        Ok(res)
    };
    fn confirmation(req: &mut Request) -> IronResult<Response> {

        //Gets the Encoded URL from the POST, Check with match, and get user var and print
        let payload = req.get_ref::<UrlEncodedBody>();
        match payload {
            Ok(ref hashmap) => {println!("Parsed POST request body:\n {:?}", hashmap);},
            Err(ref e) => println!("{:?}", e)
        };
        let payload = payload.unwrap();
        let firstname = &payload["firstname"][0];

        //Setup and send email
        email::create(
            "to@example.com",
            "from@example.com",
            "Subject - Post Variables",
            firstname,
        );
        email::send("alex.steven.elias@gmail.com");

        //Setup page variables
        let mut res = Response::new();
        let mut data = BTreeMap::new();
        data.insert("title".to_string(), "Confirmation".to_json());

        //Use same page i.e. (contact) or use new page i.e. (submit)
        res.set_mut(Template::new("confirmation", data)).set_mut(status::Ok);
        Ok(res)
    };
    //Routes


    //Add routes to router
    router.get("/", index).get("/contact", contact).post("/confirmation", confirmation);

    //Chaing link the router
    let mut chain = Chain::new(router);
    //Define where the files are located(./views/) and add the extension(.hbs)
    chain.link_after(HandlebarsEngine::new("./views/", ".hbs"));

    //Add Chain to the Mounts and add a Static directory
    let mut mounts = Mount::new();
    mounts
        .mount("/", chain)
        .mount("/public", Static::new(Path::new("public")) );


    //Add Mounts to the server
    Iron::new( mounts ).http("localhost:8080").unwrap();
}
