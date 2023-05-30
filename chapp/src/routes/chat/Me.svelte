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

<section>
	<div>
		Hi {me?.name || me?.id}, you can update your name below
	</div>
	<form on:submit|preventDefault={onClick}>
		<input bind:value={name} />
		<button type="submit"> Update name </button>
  </form>
</section>
