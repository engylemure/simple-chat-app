<script lang="ts">
	import client from '../../../graphql/client';
	import {
		Room,
		RoomMessages,
		SendMessage,
		type RoomMessagesSubscription,
		type Message as MessageGQL,
		type MessageConnection,
		AsyncRoom
	} from '../../../graphql/generated';
	import type { PageServerData } from './$types';
	import Message from './Message.svelte';
	import debounce from 'lodash/debounce';
	export let data: PageServerData;
	let roomQuery = data.room?.id
		? Room({
				variables: { id: data.room.id, last: data.lastAmount },
				fetchPolicy: 'cache-and-network'
		  })
		: undefined;
	let messagesSubscription = data.room?.id
		? RoomMessages({ variables: { id: data.room.id } })
		: undefined;
	let repliedTo: MessageGQL | undefined;
	$: room = $roomQuery?.data.room || data.room;
	$: messagesConnection = room?.messages;
	$: messages = messagesConnection?.nodes || data.room?.messages.nodes || [];
	$: reversedMessages = [...messages].reverse();
	$: onMessage($messagesSubscription?.data);
	const onMessage = (messageSubscriptionData: RoomMessagesSubscription | null | undefined) => {
		const newMessage = messageSubscriptionData?.roomMessages;
		if (newMessage) {
			updateCache(newMessage);
		}
	};
	let message: string = '';
	const sendMessage = async () => {
		if (room && message) {
			await SendMessage({
				variables: { input: { roomId: room.id, content: message, repliedTo: repliedTo?.id } },
				context: { headers: { user_id: data.user?.id } }
			});
			message = '';
			repliedTo = undefined;
		}
	};
	const updateCache = (message: MessageGQL) => {
		if (room) {
			client.cache.modify({
				id: client.cache.identify(room),
				fields: {
					messages(cachedMessages: MessageConnection) {
						return {
							...cachedMessages,
							nodes: [...cachedMessages.nodes, message]
						} satisfies MessageConnection;
					}
				}
			});
		}
	};
	const onScroll = debounce(async () => {
		const element = document.getElementById('messages');
		if (element && messagesConnection && room) {
			if (element.scrollTop == 0 && messagesConnection.pageInfo.hasNextPage) {
				const resp = await AsyncRoom({
					variables: { id: room.id, after: reversedMessages[0].id, first: data.lastAmount },
					fetchPolicy: 'network-only'
				});
				if (resp.data?.room) {
					const room = resp.data?.room;
					client.cache.modify({
						id: client.cache.identify(room),
						fields: {
							messages(cachedMessages: MessageConnection) {
								return {
									...cachedMessages,
									pageInfo: {
										...cachedMessages.pageInfo,
										hasNextPage:
											data.room?.messages.pageInfo.hasNextPage ||
											cachedMessages.pageInfo.hasNextPage
									},
									nodes: [...cachedMessages.nodes, ...room.messages.nodes]
								} satisfies MessageConnection;
							}
						}
					});
				}
			} else if (
				Math.abs(element.scrollTop - 1) + element.offsetHeight >= element.scrollHeight &&
				messagesConnection.pageInfo.hasPreviousPage
			) {
				const resp = await AsyncRoom({
					variables: { id: room.id, before: messages[0].id, last: data.lastAmount },
					fetchPolicy: 'network-only'
				});
				const roomResponse = resp.data.room;
				if (roomResponse) {
					client.cache.modify({
						id: client.cache.identify(roomResponse),
						fields: {
							messages(cachedMessages: MessageConnection) {
								return {
									...cachedMessages,
									pageInfo: {
										...cachedMessages.pageInfo,
										hasPreviousPage:
											roomResponse.messages.pageInfo.hasPreviousPage !== undefined
												? roomResponse.messages.pageInfo.hasPreviousPage
												: cachedMessages.pageInfo.hasPreviousPage
									},
									nodes: [...roomResponse.messages.nodes, ...cachedMessages.nodes]
								} satisfies MessageConnection;
							}
						}
					});
				}
			}
		}
	}, 300);
	const onMessageClick = (message: MessageGQL) => {
		repliedTo = message;
	};
</script>

<div class="content">
	<h1>{room?.name || 'No name'}</h1>
	<h3>
		number of messages {messages.length} hasPreviousPage: {messagesConnection?.pageInfo
			.hasPreviousPage} hasNextPage: {messagesConnection?.pageInfo.hasNextPage}
	</h3>
	<div id="messages" class="messages" on:scroll={onScroll}>
		{#each reversedMessages as message}
			<Message data={message} isOwn={data.user?.id === message.user?.id} onClick={onMessageClick} />
		{/each}
	</div>
</div>
<footer>
	{#if repliedTo}
		<div
			class="repliedTo"
			on:click={() => {
				repliedTo = undefined;
			}}
            on:keyup={() => {
				repliedTo = undefined;
			}}
		>
            <strong>{repliedTo.user?.name || repliedTo.user?.id}</strong>
            <br />
            <span>{repliedTo.content}</span>
		</div>
	{/if}
	<form on:submit|preventDefault={sendMessage}>
		<input bind:value={message} />
		<button type="submit"> Send message </button>
	</form>
</footer>

<style>
	.content {
		display: flex;
		flex-direction: column;
		height: 100%;
		flex: 1;
		overflow-y: auto;
	}
	.messages {
		overflow-y: auto;
		display: flex;
		flex-direction: column-reverse;
		gap: 8px;
		padding: 1rem 0.5rem;
	}
	footer {
		display: flex;
		align-items: center;
		flex-direction: column;
		position: sticky;
		padding: 2rem;
		bottom: 0;
		left: 0;
	}
</style>
