<script lang="ts">
	import { fly } from 'svelte/transition';
	import client from '../../../graphql/client';
	import {
		Room,
		RoomMessages,
		SendMessage,
		type RoomMessagesSubscription,
		type Message as MessageGQL,
		type MessageConnection,
		AsyncRoom,
		RoomDoc
	} from '../../../graphql/generated';
	import type { PageServerData } from './$types';
	import Message from './Message.svelte';
	import debounce from 'lodash/debounce';
	import isEmpty from 'lodash/isEmpty';
	import { flip } from 'svelte/animate';
	import { onMount } from 'svelte';
	export let data: PageServerData;
	onMount(() => {
		client.cache.writeQuery({
			query: RoomDoc,
			variables: { id: data.roomId, last: data.lastAmount },
			data: data
		})
	})
	const roomQuery = Room({
		variables: { id: data.roomId, last: data.lastAmount },
		fetchPolicy: 'cache-and-network',
		nextFetchPolicy: 'cache-and-network',
		refetchWritePolicy: 'merge',
	});
	const messagesSubscription = RoomMessages({ variables: { id: data.roomId } });
	let repliedTo: MessageGQL | undefined;
	$: room = isEmpty($roomQuery?.data.room) ? data.room : $roomQuery?.data.room;
	$: messagesConnection = room?.messages;
	$: messages = (messagesConnection?.nodes || data.room?.messages.nodes)!;
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
	const onMessageMount = (message: MessageGQL, i: number) => {
		if (i === 0) {
			document.getElementById(message.id)?.scrollIntoView({ behavior: 'smooth' });
		}
	};
</script>

<div class="flex flex-1 h-full flex-col overflow-y-hidden">
	<h1>{room?.name || 'No name'}</h1>
	<h3>
		number of messages {messages.length} hasPreviousPage: {messagesConnection?.pageInfo
			.hasPreviousPage} hasNextPage: {messagesConnection?.pageInfo.hasNextPage}
	</h3>
	<div
		id="messages"
		class="overflow-x-hidden overflow-y-auto flex flex-col-reverse gap-2 py-4"
		on:scroll={onScroll}
	>
		{#each reversedMessages as message, i (message)}
			{@const isOwn = data.user?.id === message.user?.id}
			{@const x = isOwn ? 100 : -100}
			<div animate:flip in:fly={{ x, duration: 400 }} out:fly={{ x, duration: 400 }}>
				<Message
					on:mount={() => onMessageMount(message, i)}
					data={message}
					{isOwn}
					onClick={onMessageClick}
				/>
			</div>
		{/each}
	</div>
</div>
<footer class="flex flex-col bg-slate-400 rounded-xl w-full gap-2">
	{#if repliedTo}
		<div
			class="flex-inline"
			transition:fly={{ y: -100, duration: 250 }}
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
	<form on:submit|preventDefault={sendMessage} class="flex flex-1 w-full flex-row gap-2">
		<input bind:value={message} class="flex text-left w-full flex-1 rounded-md px-4 py-1" />
		<button type="submit" class="text-white bg-slate-900 px-4 py-1 rounded-md">
			Send message
		</button>
	</form>
</footer>

<style>
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
