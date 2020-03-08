
#[warn(unused_imports)]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};


pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Footer {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="container">
                <div class="row">
                    <div class="col-md-12 col-lg-4">
                    <div class="footer-logo">
                        <a class="navbar-brand" href="#">{"eForum"}</a>
                        <p>{"Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the. Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry."}</p>
                    </div>
                    </div>
                    <div class="col-sm-6 col-md-3 col-lg-2">
                    <div class="list-menu">
                        <h4>{"About Us"}</h4>
                        <ul class="list-unstyled">
                        <li><a href="#">{"Link 01"}</a></li>
                        <li><a href="#">{"Link 02"}</a></li>
                        <li><a href="#">{"Link 03"}</a></li>
                        <li><a href="#">{"Link 04"}</a></li>
                        </ul>
                    </div>
                    </div>
                    <div class="col-sm-6 col-md-3 col-lg-2">
                    <div class="list-menu">
                        <h4>{"Quick Links"}</h4>
                        <ul class="list-unstyled">
                        <li><a href="#">{"Link 01"}</a></li>
                        <li><a href="#">{"Link 02"}</a></li>
                        <li><a href="#">{"Link 03"}</a></li>
                        <li><a href="#">{"Link 04"}</a></li>
                        </ul>
                    </div>
                    </div>
                    <div class="col-sm-6 col-md-3 col-lg-2">
                    <div class="list-menu">
                        <h4>{"Support"}</h4>
                        <ul class="list-unstyled">
                        <li><a href="#">{"Link 01"}</a></li>
                        <li><a href="#">{"Link 02"}</a></li>
                        <li><a href="#">{"Link 03"}</a></li>
                        <li><a href="#">{"Link 04"}</a></li>
                        </ul>
                    </div>
                    </div>
                    <div class="col-sm-6 col-md-3 col-lg-2">
                    <div class="list-menu">
                        <h4>{"Privacy"}</h4>
                        <ul class="list-unstyled">
                        <li><a href="#">{"Link 01"}</a></li>
                        <li><a href="#">{"Link 02"}</a></li>
                        <li><a href="#">{"Link 03"}</a></li>
                        <li><a href="#">{"Link 04"}</a></li>
                        </ul>
                    </div>
                    </div>
                </div>
                </div>
                <div class="copyrights">
                <div class="container">
                    <p>{"Copyright &copy; eForum - All rights reserved."}</p>
                    <div class="credits">
                    {" Designed by "} <a href="#">{"XYZ Company"}</a>
                    </div>
                </div>
                </div>
            </footer>
        }
    }
 }

