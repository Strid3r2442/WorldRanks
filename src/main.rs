mod components;
mod types;

use dioxus::prelude::*;
use components::{Wrapper, CountryDetails, CountryList};
use types::CCA3;

#[derive(Routable, PartialEq, Clone)]
enum Route {
	#[layout(Wrapper)]
	#[route("/:cca3")]
	CountryDetails { cca3: CCA3 },
	#[route("/:..segments")]
	CountryList {
		segments: Vec<String>
	},
}

pub static TITLE:GlobalSignal<String> = Signal::global(|| "Home".to_string());

fn main() {
	dioxus::launch(App);
}

const FAVICON: Asset = asset!("assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("assets/tailwind.css");

#[component]
fn App() -> Element {
	use_effect(|| {
		let page_title = TITLE();
		if let Some(window) = web_sys::window() {
			window.document().unwrap().set_title(format!(
				"{page_title} | WorldRanks"
			).as_str());
		};
	});

	rsx! {
		document::Link {
			rel: "icon",
			href: FAVICON,
			type: "image/x-icon",
			sizes: "96x96"
		}
		document::Link {
			rel: "preconnect",
			href: "https://fonts.googleapis.com"
		}
		document::Link {
			rel: "preconnect",
			href: "https://fonts.gstatic.com",
			crossorigin: "anonymous"
		}
		document::Stylesheet {
			href: "https://fonts.googleapis.com/css2?family=Be+Vietnam+Pro:wght@400;500;600;700&display=swap"
		}
		document::Stylesheet {
			href: TAILWIND_CSS
		}
		Router::<Route> { }
	}
}

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
