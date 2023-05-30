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
{#if rooms}
	<ul>
		{#each rooms as room}
			<li>
				<a href={`/chat/${room.id}`}>{room.name}</a>
			</li>
		{/each}
	</ul>
{/if}

<form on:submit|preventDefault={createRoom}>
	<input bind:value={name} />
	<button type="submit"> Create Room </button>
</form>
