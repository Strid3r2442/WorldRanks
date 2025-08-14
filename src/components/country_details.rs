use dioxus::prelude::*;
use gloo_net::http::Request;
use crate::{types::{
	Country,
	NeighbouringCountry,
	CCA3
}, TITLE};
use thousands::Separable;
use crate::Route;

#[component]
pub fn CountryDetails(cca3: CCA3) -> Element {
	let country_resource = use_resource(
		use_reactive!(|cca3| async move {
			let vec = Request::get(
				&format!(
					"https://restcountries.com/v3.1/alpha/{cca3}"
				))
				.send()
				.await
				.unwrap()
				.json::<Vec<Country>>()
				.await
				.unwrap();
	
			let country = vec.first().unwrap().clone();
			*TITLE.write() = country.name.common.to_string();
			
			country
		})
	);

	let neighbour_resource = use_resource(move || {
		let country = country_resource.read().clone();
		async move {
			if let Some(country) = country {
				match country.borders.as_ref() {
					Some(borders) if !borders.is_empty() => {
						let codes = borders
							.iter()
							.map(|code| code.as_str())
							.collect::<Vec<_>>()
							.join(",");

						let vec = Request::get(
							&format!(
								"https://restcountries.com/v3.1/alpha?fields=name,flags,cca3&codes={codes}"
							),
						)
						.send()
						.await
						.unwrap()
						.json::<Vec<NeighbouringCountry>>()
						.await
						.unwrap();

						Some(vec)
					}
					_ => Some(Vec::<NeighbouringCountry>::new()),
				}
			} else {
					None
			}
    }
	});

	rsx! {
		main {
			class: "flex justify-center items-center h-max mt-[-4rem]",
			div {
				class: "w-[95%] xl:w-1/2 flex flex-col justify-center items-center bg-darker border border-dark rounded-2xl gap-y-5 pb-5",
				match country_resource() {
					None => rsx! {
						div {
							class: "w-1/3 mt-[-5%] rounded-md bg-dark h-52 animate-pulse"
						}
					},
					Some(country) => rsx! {
						img {
							class: "w-1/3 mt-[-5%] rounded-md",
							src: "{country.flags.svg}"
						}
					}
				}
				section {
					class: "text-center w-1/3",
					match country_resource() {
						None =>	rsx! {
							div {
								class: "w-full h-9 bg-dark rounded-2xl animate-pulse mb-2"
							}
							div {
								class: "w-full h-5 bg-dark rounded-2xl animate-pulse"
							}
						},
						Some(country) => rsx! {
							h1 {
								class: "text-4xl font-semibold",
								"{country.name.common}"
							}
							h3 {
								class: "text-lg",
								"{country.name.official}"
							}
						}
					}
				}
				section {
					class: "flex flex-row justify-evenly w-full px-5",
					div {
						class: "bg-dark p-3 rounded-2xl has-[.loading]:animate-pulse",
						match country_resource() {
							None => rsx! {
								span {
									class: "block w-48 loading"
								}
							},
							Some(country) => rsx! {
								span {
									class: "border-r border-darker pr-2 py-1",
									"Population"
								}
								span {
									class: "pl-2",
									"{country.population.separate_with_commas()}"
								}
							}
						}
					}
					div {
						class: "bg-dark p-3 rounded-2xl has-[.loading]:animate-pulse",
						match country_resource() {
							None => rsx! {
								span {
									class: "block w-48 loading"
								}
							},
							Some(country) => rsx! {
								span {
									class: "border-r border-darker pr-2 py-1",
									"Area(km²)"
								}
								span {
									class: "pl-2",
									"{country.area.separate_with_commas()}"
								}
							}
						}
					}
				}
				section {
					class: "w-full",
					div {
						class: "border-t border-dark flex flex-row justify-between p-5",
						span {
							"Capital"
						}
						match country_resource() {
							None => rsx! {
								span {
									class: "bg-dark w-40 rounded-md animate-pulse"
								}
							},
							Some(country) =>  {
								match country.capital {
									Some(capital) => {
										let mut capitals: Vec<String> = capital;
										capitals.sort();
										rsx! {
											span {
												"{capitals.join(\", \")}"
											}
										}
									},
									None => rsx! {
										span {
											"No data"
										}
									}
								}
							}
						}
					}
					div {
						class: "border-t border-dark w-full flex flex-row justify-between p-5",
						span {
							"Subregion"
						}
						match country_resource() {
							None => rsx! {
								span {
									class: "bg-dark w-40 rounded-md animate-pulse"
								}
							},
							Some(country) => {
								match country.sub_region {
									Some(sub_region) => rsx! {
										span {
											"{sub_region}"
										}
									},
									None => rsx! {
										span {
											"No data"
										}
									}
								}
							}
						}
					}
					div {
						class: "border-t border-dark w-full flex flex-row justify-between p-5",
						span {
							"Language"
						}
						match country_resource() {
							None => rsx! {
								span {
									class: "bg-dark w-40 rounded-md animate-pulse"
								}
							},
							Some(country) =>  {
								match country.languages {
									Some(languages) => {
										let mut languages: Vec<String> = languages
											.values()
											.cloned()
											.collect::<Vec<String>>();
										languages.sort();
										rsx! {
											span {
												"{languages.join(\", \")}"
											}
										}
									},
									None => rsx! {
										span {
											"No data"
										}
									}
								}
							}
						}
					}
					div {
						class: "border-t border-dark w-full flex flex-row justify-between p-5",
						span {
							"Currencies"
						}
						match country_resource() {
							None => rsx! {
								span {
									class: "bg-dark w-40 rounded-md animate-pulse"
								}
							},
							Some(country) =>  {
								match country.currencies {
									Some(currencies) => {
										let mut currencies: Vec<String> = currencies
											.values()
											.map(|c| c.name.clone())
											.collect();
										currencies.sort();
										rsx! {
											span {
												"{currencies.join(\", \")}"
											}
										}
									},
									None => rsx! {
										span {
											"No data"
										}
									}
								}
							}
						}
					}
					div {
						class: "border-y border-dark w-full flex flex-row justify-between p-5",
						span {
							"Continents"
						}
						match country_resource() {
							None => rsx! {
								span {
									class: "bg-dark w-40 rounded-md animate-pulse"
								}
							},
							Some(country) => rsx! {
								span {
									"{country.region}"
								}
							}
						}
					}
				}
				section {
					class: "w-full px-5 flex flex-col gap-5",
					span {
						"Neighbouring countries"
					}
					div {
						class: "flex flex-row flex-wrap gap-5 justify-center",
						match neighbour_resource() {
							Some(Some(neighbours)) if !neighbours.is_empty() => rsx! {
								for neighbour in neighbours.iter() {
									div {
										class: "1/5 md:w-1/6",
										Link {
											to: Route::CountryDetails { cca3: neighbour.cca3 },
											img {
												class: "h-12 md:h-15 mx-auto object-cover mb-1 rounded-md",
												src: "{neighbour.flags.svg}"
											}
											span {
												class: "block text-center",
												"{neighbour.name.common}"
											}
										}
									}
								}
							},
							Some(Some(neighbours)) if neighbours.is_empty() => rsx! {
								span {
									"No neighbours"
								}
							},
							_ => rsx! {
								for _ in 0..5 {
									div {
										class: "w-1/6",
										div {
											class: "w-25 h-15 mx-auto bg-dark mb-2 rounded-md animate-pulse"
										}
										div {
											class: "w-25 mx-auto h-[1rem] rounded-2xl bg-dark animate-pulse"
										}
									}
								}
							},
						}
					}
				}
			}
		}
	}
}
