#[warn(unused_imports)]
use yew::prelude::*;
use yew_router::{prelude::*, Switch, switch::Permissive , route::Route};



#[path = "navbar.rs"] mod navbar;
use self::navbar::Navbar;
#[path = "footer.rs"] mod footer;
use self::footer::Footer;
pub struct Login;



impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Login {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        html! {
            <>
            <Navbar/>
            <br/><br/><br/><br/><br/><br/><br/><br/>
<div class="container mt-5">
        <div class="row centered-form">
        <div class="col-xs-12 col-sm-8 col-md-4 col-sm-offset-2 col-md-offset-4">
        	<div class="panel panel-default">
        		<div class="panel-heading">
			    		<h3 class="panel-title">{"Login"} <small>{"It's free!"}</small></h3>
			 			</div>
			 			<div class="panel-body">
			    		<form role="form">
			    			<div class="row">
			    				<div class="col-xs-6 col-sm-6 col-md-6">
			    				</div>
			    			</div>

			    			<div class="form-group">
			    				<input type="email" name="email" id="email" class="form-control input-sm" placeholder={"Email Address"} />
			    			</div>

			    			<div class="row">
			    				<div class="col-xs-6 col-sm-6 col-md-6">
			    					<div class="form-group">
			    						<input type="password" name="password" id="password" class="form-control input-sm" placeholder={"Password"} />
			    					</div>
			    				</div>
			    			</div>
			    			
			    			<input type="submit" value={"LOGIN"} class="btn btn-info btn-block" />
			    		
			    		</form>
			    	</div>
	    		</div>
    		</div>
    	</div>
    </div>
    <Footer/>
    </>
                
               }
           
           }
       }
 