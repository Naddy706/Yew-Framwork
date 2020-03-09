#[warn(unused_imports)]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};



pub struct Navbar;



impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Navbar {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        html! {

          
  <header id="header" class="header header-hide">
    <div class="top-bar">
      <div class="container">
        <div class="row">
          <ul class="top-menu">
            <li class=""><a href="/">{"Questions"}</a></li>
            <li><a href="/about">{"About"}</a></li>
          </ul>
        </div>
      </div>
    </div>
    <div class="container">
      <div id="logo" class="pull-left">
        <h1><a href="#" class="scrollto"><span>{"e"}</span>{"Forum"}</a></h1>
      </div>
      <nav id="nav-menu-container">
        <ul class="nav-menu">
          <li class="menu-active"><a href="/login">{"Login"}</a></li>
          <li><a href="/register">{"Register"}</a></li>
        </ul>
      </nav>
    </div>
  </header>
        
                
               }
           
           }
       }
 