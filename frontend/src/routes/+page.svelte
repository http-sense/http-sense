<script lang="ts">
	import { get_api_url, get_ws_url } from "$lib/utils";
	import { onMount } from "svelte";
	import { merge_ssr_styles } from "svelte/internal";

	let api_url = get_api_url();

	type Method = string;
	type Uri = string;
	type Body = string;
	type Headers = {[key: string]: string};
	type Status = number;
	type UUIDS = string;

	interface RequestStart {
		body: Body,
		headers: Headers,
		id: UUIDS,
		method: Method,
		uri: Uri
	}
	interface ResponseComplete {
		body: Body,
		headers: Headers,
		id: UUIDS,
		status_code: Status,
	}
	interface ResponseError {
		id: UUIDS,
		error: string
	}

	interface Reqeust {
		request: RequestStart,
		response?: ResponseComplete | ResponseError
	}

	let history: {[key in UUIDS]: Reqeust} = {};

	async function init_data() {
		const ws = new WebSocket(get_ws_url())
		ws.onmessage = function message(event) {
			let msg = JSON.parse(event.data) as RequestStart | ResponseComplete | ResponseError;
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
			console.log(entry)
			entry.request.id = entry.id;
			if (!history[entry.request_id]) {
				history[entry.request_id] = {request: entry.request}
			}
			history[entry.request_id].response = entry.response || history[entry.request_id].response
		}
		history = history
		console.log(data)
	}
	onMount(init_data)

</script>
<pre>
  <code>
    Please send a GET request to /api/requests endpoint until we scrap up our frontend
    <!-- {#each requests as request}
      {JSON.parse(request.request_content).uri}
      <br />
    {/each} -->
    {#each Object.entries(history) as [id, req]}
      {id}: {req.request.uri} - {req.response?.id}
      <br >
    {/each}
  </code>
</pre>

