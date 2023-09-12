#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {


    let test = "this string is a variable";

    cx.render(rsx! {
        div {
            class: "absolute inset-0 bg-red-500 flex place-items-center justify-center text-white",
            div {
                class: "w-full max-w-3xl bg-red-400 p-5 rounded-xl",
                h1 {
                    class: "text-xl uppercase font-bold",
                    "My first website with rust"
                },
                p {class:"italic text-xs", {test}}
                p {"Hey, that's pretty cool, right?"},
                p {"What else can we do? Here's a component with something fancy, maybe."},
                CoolComponent {}
                
            }
        }
    })
}


fn CoolComponent(cx: Scope) -> Element {
    render! {
        div {
            class: "mt-2 p-4 w-full h-24 bg-white rounded-xl",
            h1 {
                class: "text-black text-2xl font-bold",
                "WOW, this is in fact cool, ", span {class: "italic" ,"but what about a span"}, " hmmm? oh it works"
            }
        }
    }
}





