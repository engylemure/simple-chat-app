<script lang="ts">
	import { Rooms, CreateRoom, RoomsDoc, type RoomsQuery } from '../../graphql/generated';
	export let data: RoomsQuery | undefined | null;
	$: roomsQuery = Rooms({});
	$: rooms = ((!$roomsQuery.loading && $roomsQuery.data) || data)?.rooms;
	let name: string;
	const createRoom = async () => {
		const createdRoom = (await CreateRoom({ variables: { name }, refetchQueries: [RoomsDoc] })).data?.createRoom;
        if (createdRoom) {
            document.location.assign(`/chat/${createdRoom.id}`)
        }
	};
</script>

<h1>Rooms</h1>

<form class="rounded-xl md:flex flex-row max-w-xl gap-4"  on:submit|preventDefault={createRoom}>
	<input class="rounded-md" bind:value={name} />
	<button class="bg-slate-900 text-white py-1 px-3 rounded-md" type="submit"> Create Room </button>
</form>

{#if rooms}
	<ul class="md:flex flex-col gap-4 mt-4">
		{#each rooms as room}
			<li>
				<a href={`/chat/${room.id}`} class="text-purple-900">{room.name}</a>
			</li>
		{/each}
	</ul>
{/if}


