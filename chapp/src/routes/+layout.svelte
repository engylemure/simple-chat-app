<script>
	import Header from './Header.svelte';
	import './styles.css';
</script>

<div class="h-screen w-screen overflow-y-auto min-h-screen max-h-screen flex flex-1 flex-col">
	<Header />

	<main class="flex flex-1 flex-col p-1 w-full h-full mx-auto my-0">
		<slot />
	</main>
</div>

<style>

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		width: 100%;
		margin: 0 auto;
		box-sizing: border-box;
		position: relative;
		overflow-y: auto;
	}
</style>
