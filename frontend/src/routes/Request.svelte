<script lang="ts">
	import type * as m from './models';
	export let id: string;
	export let request: m.Reqeust;
</script>

<div>
	{id}: {request.request.uri} - {request.response?.id}
</div>