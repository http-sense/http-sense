import { browser } from "$app/environment";
import { get } from 'svelte/store';
import { page } from "$app/stores";

export function get_api_url(): URL {
	let page_url = get(page).url;
	let _base_api_url = page_url.origin + "/api";
	let api_url = page_url.searchParams.get('api_url')
	if (api_url) {
		return new URL(api_url);
	}
	return new URL(_base_api_url);
}