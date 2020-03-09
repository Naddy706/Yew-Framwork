#[macro_use]

use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};



#[path = "home.rs"] mod home;
use self::home::Home;

#[path = "about.rs"] mod about;
use self::about::About;


#[path = "register.rs"] mod register;
use self::register::Register;


#[path = "login.rs"] mod login;
use self::login::Login;



pub struct App;


#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to= "/!"]
    RootPath,
    #[to= "/about"]
    AboutPath,
    #[to= "/register"]
    RegisterPath,
    #[to= "/login"]
    LoginPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{<Home/>},
                            AppRouter::AboutPath => html!{<About/>},
                            AppRouter::RegisterPath => html!{<Register/>},
                            AppRouter::LoginPath => html!{<Login/>},
                            AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
        }
    }
 }


 


 