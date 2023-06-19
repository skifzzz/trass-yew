mod gateway;
mod recommendations;

use core::panic;
use std::ops::Deref;

use gloo_net::http::Request;

use gloo_console::log;

use yew::{prelude::*, platform::spawn_local};

use crate::app::recommendations::{data::Recommendations, components::RecommendationsComponent};

#[function_component(App)]
pub fn app() -> Html {

    let recommendations = use_state(Recommendations::default);
    {
        let recommendations = recommendations.clone();
        use_effect_with_deps(move |_| {
            let recommendations = recommendations.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let request = Request::get("http://localhost:3000/recommendation")
                .mode(web_sys::RequestMode::Cors);

                let maybe_response = request
                .send()
                .await;

                let fetched = match maybe_response {
                    Ok(response) => {
                        log!("Response returned");
                        response.json()
                            .await
                            .unwrap()
                    },
                    Err(e) => {
                        log!(format!("Can't get response, error: {:?}", e));
                        Recommendations::default()
                    }
                };

                recommendations.set(fetched)
            });
            || ()
        }, ());
    }

    let html = html! {
        <>
        // <div class="container-fluid">
        // // <nav class="nav nav-tabs nav-fill">
        // //         <a class="nav-link active" aria-current="page" href="#">{"Active"}</a>
        // //         <a class="nav-link" href="#">{"Much longer nav link"}</a>
        // //         <a class="nav-link" href="#">{"Link"}</a>
        // //         <a class="nav-link disabled">{"Disabled"}</a>
        // //     </nav>
        //     <div></div>
        //     <nav>
        //         <div class="nav nav-tabs" id="nav-tab" role="tablist">
        //             <button class="nav-link active" id="nav-home-tab" data-bs-toggle="tab" data-bs-target="#nav-home" type="button" role="tab" aria-controls="nav-home" aria-selected="true">{"Home"}</button>
        //             <button class="nav-link" id="nav-profile-tab" data-bs-toggle="tab" data-bs-target="#nav-profile" type="button" role="tab" aria-controls="nav-profile" aria-selected="false">{"Profile"}</button>
        //             <button class="nav-link" id="nav-contact-tab" data-bs-toggle="tab" data-bs-target="#nav-contact" type="button" role="tab" aria-controls="nav-contact" aria-selected="false">{"Contact"}</button>
        //             <button class="nav-link" id="nav-disabled-tab" data-bs-toggle="tab" data-bs-target="#nav-disabled" type="button" role="tab" aria-controls="nav-disabled" aria-selected="false" disabled=true>{"Disabled"}</button>
        //         </div>
        //         </nav>
        //         <div class="tab-content" id="nav-tabContent">
        //         <div class="tab-pane fade show active" id="nav-home" role="tabpanel" aria-labelledby="nav-home-tab" tabindex="0">{".11111.."}</div>
        //         <div class="tab-pane fade" id="nav-profile" role="tabpanel" aria-labelledby="nav-profile-tab" tabindex="0">{".222222.."}</div>
        //         <div class="tab-pane fade" id="nav-contact" role="tabpanel" aria-labelledby="nav-contact-tab" tabindex="0">{".333333.."}</div>
        //         <div class="tab-pane fade" id="nav-disabled" role="tabpanel" aria-labelledby="nav-disabled-tab" tabindex="0">{".444444.."}</div>
        //         </div>


        //     </div>

        // <div class=".container">
        <div class={classes!("container")}>
        <h3>{"Recommendations start"}</h3>

        <RecommendationsComponent recommendations={recommendations.deref()}/>

        <h3>{"End"}</h3>

        <ul class="nav nav-tabs">
        <li class="active"><a data-toggle="tab" href="#home">{"Home"}</a></li>
        <li><a data-toggle="tab" href="#menu1">{"Menu 1"}</a></li>
        <li><a data-toggle="tab" href="#menu2">{"Menu 2"}</a></li>
        </ul>

        <div class="tab-content">
        <div id="home" class="tab-pane fade in active">
            <h3>{"HOME"}</h3>
            <p>{"Some content."}</p>
        </div>
        <div id="menu1" class="tab-pane fade">
            <h3>{"Menu 1"}</h3>
            <p>{"Some content in menu 1."}</p>
        </div>
        <div id="menu2" class="tab-pane fade">
            <h3>{"Menu 2"}</h3>
            <p>{"Some content in menu 2."}</p>
        </div>
        </div>

        </div>


        // <main>
        //     <div>{"Response from api:"}</div>
        //     <br/>
        //     // <div> {just_string}</div>
        //     <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
        //     <h1>{ "Hello World!" }</h1>
        //     <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>

            
        // </main>
        // <nav class="nav nav-pills nav-justified">
        //         <a class="nav-link active" aria-current="page" href="#">{"Active"}</a>
        //         <a class="nav-link" href="#">{"Much longer nav link"}</a>
        //         <a class="nav-link" href="#">{"Link"}</a>
        //         <a class="nav-link disabled">{"Disabled"}</a>
        //     </nav>
        </>
    };

    html
}
