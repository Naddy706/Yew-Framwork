#[warn(unused_imports)]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};



pub struct GetStarted;

impl Component for GetStarted {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    GetStarted {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
        <section id="get-started" class="padd-section text-center wow fadeInUp">
    <div class="container">
      <div class="section-title text-center">
        <h2>{"About eForum"}</h2>
        <p class="separator">{"Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit magni dolores eos qui."}</p>
      </div>
    </div>
    <div class="container">
      <div class="row">
        <div class="col-md-6 col-lg-4">
          <div class="feature-block">
            <img src="img/svg/vector.svg" alt="img" class="img-fluid" />
            <h4>{"introducing eForum"}</h4>
            <p>{"Lorem Ipsum is simply dummy text of the printing and typesetting industry"}</p>
          </div>
        </div>
        <div class="col-md-6 col-lg-4">
          <div class="feature-block">
            <img src="img/svg/design-tool.svg" alt="img" class="img-fluid" />
            <h4>{"user friendly interface"}</h4>
            <p>{"Lorem Ipsum is simply dummy text of the printing and typesetting industry"}</p>
          </div>
        </div>
        <div class="col-md-6 col-lg-4">
          <div class="feature-block">
            <img src="img/svg/pixel.svg" alt="img" class="img-fluid" />
            <h4>{"contribute at eForum"}</h4>
            <p>{"Lorem Ipsum is simply dummy text of the printing and typesetting industry"}</p>
          </div>
        </div>
      </div>
    </div>
  </section>
        }
    }
 }

