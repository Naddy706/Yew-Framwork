#[warn(unused_imports)]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};



pub struct Video;

impl Component for Video {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Video {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <section id="videos" class="padd-section text-center wow fadeInUp">
    <div class="container">
      <div class="section-title text-center">
        <h2>{"Forum Videos"}</h2>
                <p class="separator">{"Sed ut perspiciatis unde omnis iste natus error sit voluptatem."}</p>
      </div>
    </div>
    <div class="container">
      <div class="owl-carousel owl-theme">
        <div><iframe width="100%" height="500" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe></div>
        <div><iframe width="100%" height="500" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe></div>
        <div><iframe width="100%" height="500" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe></div>
        <div><iframe width="100%" height="500" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe></div>
        <div><iframe width="100%" height="500" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe></div>
        <div><iframe width="100%" height="500" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe></div>
      </div>
    </div>
    <div class="container">
      <div class="row padd-section">
        <div class="col-md-6 col-lg-4">
          <iframe width="100%" height="300" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe>
        </div>
        <div class="col-md-6 col-lg-4">
          <iframe width="100%" height="300" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe>
        </div>
        <div class="col-md-6 col-lg-4">
          <iframe width="100%" height="300" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe>
        </div>
        <div class="col-md-6 col-lg-4">
          <iframe width="100%" height="300" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe>
        </div>
        <div class="col-md-6 col-lg-4">
          <iframe width="100%" height="300" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe>
        </div>
        <div class="col-md-6 col-lg-4">
          <iframe width="100%" height="300" src="https://www.youtube.com/embed/W-_X1HZM7ys" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" ></iframe>
        </div>
      </div>
    </div>
  </section>
        }
    }
 }

