#[warn(unused_imports)]
#[macro_use]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};



#[path = "navbar.rs"] mod navbar;
use self::navbar::Navbar;
#[path = "footer.rs"] mod footer;
use self::footer::Footer;
#[path ="contact.rs"] mod contact;
use self::contact::Contact;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    About {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
            <Navbar/>
            
            <Contact/>
            <Footer/>
        </>
        }
    }
 }

