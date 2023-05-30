<script lang="ts">
	import type { PageData } from "./$types";
	import Me from "./Me.svelte";
	import Rooms from "./Rooms.svelte";

  export let data: PageData;
</script>

<svelte:head>
	<title>Chat</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<Me data={data.user} />
<Rooms data={data.rooms.data} />