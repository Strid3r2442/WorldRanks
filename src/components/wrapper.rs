use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Wrapper() -> Element {
	rsx! {
		Header { }
		Outlet::<Route> { }
		Footer { }
	}
}

#[component]
fn Header() -> Element {
	rsx! {
		div {
			background_image: "url({asset!(\"/assets/hero-image.jpg\")})",
			class: "w-full h-[300px] flex bg-cover z-1",
			Link {
				class: "m-auto",
				to: Route::CountryList { segments: Vec::<String>::new() },
				img {
					src: asset!("/assets/Logo.svg")
				}
			}
		}
	}
}

#[component]
fn Footer() -> Element {
	rsx! {
		div {
			class: "text-sm text-center mt-4 text-neutral-700",
			"Coded by ",
			Link {
				to: "https://github.com/Strid3r2442",
				"Jack Comer"
			},
			" | Challenge by ",
			Link {
				to: "https://www.devchallenges.io?ref=challenge",
				new_tab: true,
				"devChallenges.io"
			},
			"."
		}
	}
}
