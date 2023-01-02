<script lang="ts">
	import { get_api_url, get_ws_url } from "$lib/utils";
	import { onMount } from "svelte";
	import { merge_ssr_styles } from "svelte/internal";
	import type * as m from './models';
	import Request from "./Request.svelte";

	let api_url = get_api_url();



	let history: {[key in m.UUIDS]: m.Reqeust} = {};

	async function init_data() {
		const ws = new WebSocket(get_ws_url())
		ws.onmessage = function message(event) {
			let msg = JSON.parse(event.data) as m.RequestStart | m.ResponseComplete | m.ResponseError;
			console.log('ws msg recvd', msg);
			if ("uri" in msg) {
				history[msg.id] =  {request: msg}
			} else {
				history[msg.id].response =  msg
			}
		}
		let resp = await fetch(api_url + '/requests')

		let data = await resp.json();
		for (const entry of data) {

			entry.request.id = entry.request_id;
			if (entry.response) {
				entry.response.id = entry.request_id;
			}
			if (!history[entry.request_id]) {
				history[entry.request_id] = {request: entry.request}
			}
			history[entry.request_id].response = entry.response || history[entry.request_id].response
		}
		history = history
		console.log('history', history)
	}
	onMount(init_data)

</script>
<pre>
  <code>
    Please send a GET request to /api/requests endpoint until we scrap up our frontend
    {#each Object.entries(history) as [id, req]}
	<Request {id} request={req} />
      <br >
    {/each}
  </code>
</pre>

