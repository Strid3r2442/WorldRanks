use std::str::FromStr;
use std::collections::HashMap;
use dioxus::prelude::*;
use gloo_net::http::Request;
use strum::IntoEnumIterator;
use crate::{
	types::{
		CountryOverview,
		FilterQuery,
		Region,
		SortBy,
		Status
	},
	Route, TITLE
};
use thousands::Separable;

const PAGE_SIZE: usize = 15;
const CURRENT_PAGE: &str = "Current";
const TOTAL_PAGES: &str = "Total";

fn sort_data(mut countries: Vec<CountryOverview>, sort_by: &SortBy) -> Vec<CountryOverview> {
	match sort_by {
		SortBy::Area => countries.sort_by(|a, b| a.area.partial_cmp(&b.area).unwrap()),
		SortBy::Name => countries.sort_by(|a, b| a.name.common.cmp(&b.name.common)),
		SortBy::Population => countries.sort_by(|a, b| a.population.partial_cmp(&b.population).unwrap())
	}
	countries
}

fn toggle_region(mut region_signal: Signal<Vec<Region>>, region: Region) {
	if region_signal.read().contains(&region) {
		region_signal.retain(|r| r != &region);
	} else {
		region_signal.push(region);
	}
}

fn filter_data(countries: &[CountryOverview], queries: &[FilterQuery]) -> Vec<CountryOverview> {
	countries
		.iter()
		.filter(|c| {
			queries.iter().all(|query| match query {
				FilterQuery::Text(text_query) => {
					c.name.common.to_lowercase().contains(&text_query.to_lowercase()) ||
					c.region.to_string().to_lowercase().contains(&text_query.to_lowercase()) ||
					c.sub_region.to_string().to_lowercase().contains(&text_query.to_lowercase())
				}
				FilterQuery::Region(region_query) => region_query.is_empty() || region_query.contains(&c.region),
				FilterQuery::Status(status_query) => {
					let independent_query = status_query.get(&Status::Independent).unwrap();
					let un_member_query = status_query.get(&Status::UN).unwrap();
	
					(!independent_query || c.independent) &&
					( !un_member_query || c.un_member )
				}
			})
		})
		.cloned()
		.collect()
}

#[component]
pub fn CountryList(segments: Vec<String>) -> Element {
	let mut all_countries_signal = use_signal(|| Vec::<CountryOverview>::new());
	let mut search_text_signal = use_signal(|| "".to_string());
	let mut sort_by_signal = use_signal(|| SortBy::Population);
	let filter_region_signal = use_signal(|| Vec::<Region>::new());
	let mut filter_status_signal = use_signal(|| HashMap::from([
		(Status::Independent, false),
		(Status::UN, false)
	]));
	let mut page_signal = use_signal(|| HashMap::from([
		(CURRENT_PAGE, 0 as usize),
		(TOTAL_PAGES, 0 as usize)
	]));
	let navigator = use_navigator();

	*TITLE.write() = "Home".to_string();

	let countries_resource = use_resource(move || async move {
		Request::get(
			&format!(
				"https://restcountries.com/v3.1/all?fields=flags,name,population,area,region,subregion,cca3,independent,unMember"
			))
			.send()
			.await
			.unwrap()
			.json::<Vec<CountryOverview>>()
			.await
			.unwrap()
	});

	use_effect(move || {
		if let Some(countries) = countries_resource.read().clone() {
			all_countries_signal.set(countries);
		}
	});

	let sorted_countries = use_memo(move || {
    sort_data(all_countries_signal.read().clone(), &sort_by_signal.read())
	});

	let filtered_countries = use_memo(move || {
		filter_data(&sorted_countries.read(), &[
			FilterQuery::Text(&search_text_signal.read()),
			FilterQuery::Region(&filter_region_signal.read()),
			FilterQuery::Status(&filter_status_signal.read())
		])
	});

	use_effect(move || {
		let _filtere_countries = filtered_countries();
		// Reset the pagination to page 0 when list of countries changes
		page_signal.write().entry(CURRENT_PAGE).and_modify(|p| *p = 0);
	});

	let count = use_memo(move || {
		filtered_countries.read().len()
	});

	use_effect(move || {
		let count = count();
		if count == 0 {
			page_signal.write().entry(TOTAL_PAGES).and_modify(|p| *p = 0);
		} else {
			let page_count = (count + PAGE_SIZE - 1) / PAGE_SIZE;
			page_signal.write().entry(TOTAL_PAGES).and_modify(|p| *p = page_count);
		}
	});

	let paginated_countries = use_memo(move || {
		let start = page_signal().get(CURRENT_PAGE).unwrap() * PAGE_SIZE;
		let end = (start + PAGE_SIZE).min(filtered_countries().len());
		filtered_countries()[start..end].to_vec()
	});

	rsx! {
		main {
			class: "flex justify-center items-center h-max mt-[-4rem]",
			div {
				class: "w-[95%] xl:w-5/6 2xl:w-3/4 flex flex-col justify-center items-center bg-darker border border-dark rounded-2xl gap-y-5 p-5",
				section {
					class: "w-full flex flex-row justify-between items-center",
					h2 {
						class: "text-lg font-semibold",
						"Found {count} countries"
					}
					div {
						class: "bg-dark rounded-md p-2",
						label {
							class: "inline-block align-middle mr-2",
							img {
								src: asset!("/assets/Search.svg")
							}
						}
						input {
							class: "inline-block align-middle min-w-80",
							value: "{search_text_signal}",
							placeholder: "Search by Name, Region, Subregion",
							oninput: move |event| search_text_signal.set(event.value()),
						}
					}
				}
				div {
					class: "w-full flex flex-col xl:flex-row gap-5",
					section {
						class: "w-full xl:w-1/5 flex flex-col gap-5",
						div {
							label {
								class: "block text-xs",
								"Sort by"
							}
							select {
								class: "w-full border-2 border-dark rounded-md p-2",
								value: "{sort_by_signal}",
								oninput: move |event| sort_by_signal.set(SortBy::from_str(&event.value()).unwrap()),
								for sort in SortBy::iter() {
									option {
										value: "{sort}",
										"{sort}",
									}
								}
							}
						}
						div {
							span {
								class: "block text-xs",
								"Region"
							}
							div {
								class: "flex flex-row flex-wrap gap-x-4 gap-y-2",
								for region in Region::iter() {
										label {
											class: "has-checked:bg-dark cursor-pointer p-2 rounded-md",
											for: "{region}",
											"{region}"
											input {
												class: "hidden",
												id: "{region}",
												r#type: "checkbox",
												value: "{region}",
												oninput: move |_| toggle_region(filter_region_signal, region),
											}
										}
								}
							}
						}
						div {
							label {
								class: "block text-xs",
								"Status"
							}
							div {
								class: "flex flex-col gap-2",
								for status in Status::iter() {
									label {
										class: "group cursor-pointer",
										for: "{status}",
										div {
											class: "h-6 w-6 border-2 border-dark rounded-md inline-block align-middle mr-2 group-has-checked:border-interact group-has-checked:bg-interact",
											img {
												class: "hidden group-has-checked:block",
												src: asset!("/assets/Done_round.svg")
											}
										}
										input {
											class: "hidden",
											id: "{status}",
											r#type: "checkbox",
											checked: "{filter_status_signal.read().get(&status).unwrap()}",
											oninput: move |_| {
												filter_status_signal.write().entry(status).and_modify(|s| *s = !*s);
											},
										}
										span {
											class: "align-middle",
											"{status}"
										}
									}
								}
							}
						}
					}
					section {
						class: "grow",
						table {
							class: "w-full",
							thead {
								tr {
									class: "text-left border-b border-dark",
									th {
										class: "pb-2 w-1/10",
										"Flag"		
									}
									th {
										class: "pb-2 w-[22.5%]",
										"Name"
									}
									th {
										class: "pb-2 w-[22.5%]",
										"Population"
									}
									th {
										class: "pb-2 w-[22.5%]",
										"Area (km²)"
									}
									th {
										class: "pb-2 w-[22.5%]",
										"Region"
									}
								}
							}
							tbody {
								if !paginated_countries().is_empty() {
									for country in paginated_countries().iter() {
										tr {
											class: "cursor-pointer hover:bg-dark",
											onclick: {
												let cca3 = country.cca3.clone();
												move |_| { navigator.push(Route::CountryDetails { cca3: cca3 }); }
											},
											td {
												class: "py-2",
												img {
													class: "rounded-md max-h-8",
													src: "{country.flags.svg}",
													width: "48",
												}
											}
											td {
												class: "py-2",
												"{country.name.common}"
											}
											td {
												class: "py-2",
												"{country.population.separate_with_commas()}"
											}
											td {
												class: "py-2",
												"{country.area.separate_with_commas()}"
											}
											td {
												class: "py-2",
												"{country.region}"
											}
										}
									}
								} else if !all_countries_signal.is_empty() {
									tr {
										td {
											class: "text-center py-2",
											colspan: "5",
											"No countries found. Try expanding your filters."
										}
									}
								} else {
									for _ in 0..5 {
										tr {
											td {
												class: "py-2",
												div {
													class: "bg-dark h-8 w-12 rounded-md animate-pulse"
												}
											}
											td {
												class: "py-2",
												div {
													class: "bg-dark h-3 w-12 rounded-md animate-pulse"
												}
											}
											td {
												class: "py-2",
												div {
													class: "bg-dark h-3 w-12 rounded-md animate-pulse"
												}
											}
											td {
												class: "py-2",
												div {
													class: "bg-dark h-3 w-12 rounded-md animate-pulse"
												}
											}
											td {
												class: "py-2",
												div {
													class: "bg-dark h-3 w-12 rounded-md animate-pulse"
												}
											}
										}
									}
								}
							}
						}
						div {
							class: "w-full flex flex-row gap-2 justify-center text-center mt-3",
							for page in 0..*page_signal().get(TOTAL_PAGES).unwrap() {
								if page == *page_signal().get(CURRENT_PAGE).unwrap() {
									span {
										class: "bg-interact p-2 rounded-md w-6 h-6 box-content cursor-default",
										"{page + 1}"
									}
								} else {
									span {
										class: "bg-dark p-2 rounded-md w-6 h-6 box-content cursor-pointer",
										onclick: move |_| {
											page_signal.write().entry(CURRENT_PAGE).and_modify(|p| *p = page);
										},
										"{page + 1}"
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
