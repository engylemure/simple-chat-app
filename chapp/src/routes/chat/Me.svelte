<script lang="ts">
	import { Me, MeDoc, UpdateUser, type User } from '../../graphql/generated';
	export let data: User | null | undefined;
	let userId = data?.id;
	$: meQuery = Me({ context: { headers: { user_id: userId } } });
	$: me = $meQuery.data?.me || data;

	let name: string = '';
	const onClick = async () => {
		await UpdateUser({
			variables: { name },
			context: { headers: { user_id: userId } },
			refetchQueries: [MeDoc]
		});
		name = '';
	};
</script>

<section class="md:flex flex-col bg-slate-100 rounded-xl p-8 gap-4 align-center max-w-xl">
	<div class="font-medium">
		Hi <strong>{me?.name || me?.id}</strong>, you can update your name below
	</div>
	<form on:submit|preventDefault={onClick} class="md:flex flex-col gap-4">
		<input bind:value={name} />
		<button type="submit" class="md:flex bg-slate-900 rounded-md p-2 text-white justify-center"> Update name </button>
  </form>
</section>
