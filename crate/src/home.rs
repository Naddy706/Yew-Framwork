#[warn(unused_imports)]
#[macro_use]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};



#[path = "navbar.rs"] mod navbar;
use self::navbar::Navbar;
#[path = "footer.rs"] mod footer;
use self::footer::Footer;

#[path ="hero.rs"] mod hero;
use self::hero::Hero;
#[path = "getstarted.rs"] mod getstarted;
use self::getstarted::GetStarted;
#[path ="video.rs"] mod video;
use self::video::Video;
#[path ="contact.rs"] mod contact;
use self::contact::Contact;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
            <Navbar/>
            <Hero/>
            <GetStarted/>
            <Video/>
            <Contact/>
            <Footer/>
        </>
        }
    }
 }

