<script lang="ts">
	import type { Message } from '../../../graphql/generated';
	export let data: Message;
	export let isOwn: boolean;
    export let onClick: (message: Message) => void | undefined;

    $: repliedTo = data.repliedTo ? data.repliedTo : undefined;
    const onClickAtRepliedTo = () => {
        if (repliedTo) {
            document.getElementById(repliedTo.id)?.scrollIntoView({ behavior: 'smooth' })
        }
    }
</script>

<div id={data.id} class={`messageContainer  ${isOwn ? 'own' : ''}`} >
	<a class={`message ${isOwn ? 'own' : ''}`} href={`#${data.id}`} on:click={() => onClick(data)}>
		{#if repliedTo}
            <div class="repliedTo" on:click={onClickAtRepliedTo} on:keypress={onClickAtRepliedTo}>
                <b>{repliedTo.user?.name || repliedTo.user?.id}</b>
                <p>{repliedTo.content}</p>
            </div>
		{/if}
		<b>{data.user?.name || data.user?.id}</b>
		<p>{data.content}</p>
        <span>{new Date(data.createdAt).toLocaleString()}</span>
    </a>
</div>

<style>
	.messageContainer {
		display: flex;
		flex: 1;
	}
	.message {
        gap: 4px;
		flex-shrink: 1;
		flex-grow: 0;
		padding: 0.75rem 1rem;
		display: flex;
		flex-direction: column;
		border-radius: 8px;
		border: 1px solid rgba(0, 0, 0, 0.3);
		justify-content: flex-start;
        color: rgba(0, 0, 0, 0.95);
	}
    .message span {
        font-size: 12px;
    }
    .repliedTo {
        border: 1px solid rgba(0, 0, 0, 0.3);
        border-radius: 4px;
        flex-shrink: 1;
		flex-grow: 0;
        padding: 0.375rem 0.5rem;
    }
    .repliedTo b {
        font-size: 12px;
    }
    .repliedTo p {
        font-size: 12px;
    }
	.own {
		justify-content: flex-end;
		text-align: end;
	}
	p {
		margin: 0;
	}
</style>
