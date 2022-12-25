<script lang="ts">
	import { get_api_url } from "$lib/utils";


	let requests: any[] = [];
	let api_url = get_api_url();
	async function get_requests() {
		let resp = await fetch(api_url + '/requests')
    let data = await resp.json();
    console.log(data)
    requests = data;
	}
	get_requests()

</script>
<pre><code>
Please send a GET request to /api/requests endpoint until we scrap up our frontend
{#each requests as request}
  {JSON.parse(request.request_content).uri}
  <br />
{/each}
</code></pre>

