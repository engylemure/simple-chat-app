<script lang="ts">
	import { onMount } from "svelte";
	import { KQL_Me, KQL_UpdateUser } from "../../graphql/generated";
  export let data: { userId: string };
  let userId = data.userId;
  const customFetch: typeof fetch = (input, init) => {
    const headers = new Headers(init?.headers);
    if (userId) {
      headers.set('user_id', userId)
    }
    return fetch(input, { ...init, headers })
  }
  $: me = $KQL_Me.data?.me;
  onMount(() => {
    KQL_Me.queryLoad({ fetch: customFetch });
  })
  let name: string;
  const onClick = async () => {
    const user = await KQL_UpdateUser.mutate({
      fetch: customFetch,
      variables: { name }
    });
    if (user.data?.updateUser) {
      KQL_Me.patch({ me: user.data?.updateUser })
    }
    
  }
</script>

id: {me?.id}
name: {me?.name}

<input bind:value={name} />
<button on:click={onClick}>
  Update name
</button>
