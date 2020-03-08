#[warn(unused_imports)]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};

pub struct Hero;

impl Component for Hero {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Hero {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            
             <section id="hero" class="wow fadeIn">
                <div class="hero-container">
                    <h1>{"Welcome to eForum"}</h1>
                    <h2>{"Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque."}</h2>
                    <img src="../../img/heroimg.png" alt="Hero Imgs" />
                    <a href="#" class="btn-get-started scrollto">{"Get Started"}</a>
                        <div class="btns">
                        <ul>
                            <a href="#"><i class="fa fa-apple fa-3x"></i> {"App Store"}</a>
                            <a href="#"><i class="fa fa-play fa-3x"></i> {"Google Play"}</a>
                            <a href="#"><i class="fa fa-windows fa-3x"></i> {"windows"}</a>
                            </ul>
                        </div>
                        </div>
                        </section>
                
            
            
        }
    }
 }
