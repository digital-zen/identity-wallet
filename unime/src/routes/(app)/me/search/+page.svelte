<script lang="ts">
  import { goto } from '$app/navigation';

  import IconMessage from '$lib/components/molecules/IconMessage.svelte';
  import Search from '$lib/components/molecules/Search.svelte';
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';
  import { state } from '$src/stores';

  import Ghost from '~icons/ph/ghost-fill';
  import MagnifyingGlass from '~icons/ph/magnifying-glass-fill';

  let searchTerm: string | undefined;
  $: indices = $state.user_data_query;
  $: credentials = $state.credentials.filter((cred) => indices.includes(cred.id));
</script>

<div class="content-height bg-silver dark:bg-navy">
  <div class="p-4">
    <Search
      on:value={(e) => {
        searchTerm = e.detail;
        dispatch({
          type: '[User Data] Query',
          payload: {
            target: 'Credentials',
            search_term: e.detail,
          },
        });
      }}
    ></Search>
  </div>
  {#if !searchTerm}
    <div class="pt-12">
      <IconMessage
        icon={MagnifyingGlass}
        title={$LL.SEARCH.NO_QUERY.TITLE()}
        description={$LL.SEARCH.NO_QUERY.DESCRIPTION()}
      />
    </div>
  {:else if credentials.length == 0}
    <div class="pt-12">
      <IconMessage
        icon={Ghost}
        title={$LL.SEARCH.NO_RESULTS.TITLE()}
        description={$LL.SEARCH.NO_RESULTS.DESCRIPTION()}
      />
    </div>
  {:else}
    <div class="w-full space-y-2 p-5">
      <!-- using "key" to destroy & recreate the complete credentials list to enforce a refresh of logos -->
      {#key indices}
        {#each credentials as credential}
          <ListItemCard
            id={credential.id}
            title={credential.metadata.display.name ??
              credential.data.credentialSubject.achievement?.name ??
              credential.data.type.at(-1)}
            description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
            type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
            on:click={() =>
              credential.data.type.includes('OpenBadgeCredential')
                ? goto(`/badges/${credential.id}`)
                : goto(`/credentials/${credential.id}`)}
          />
        {/each}
      {/key}
    </div>
  {/if}
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }
</style>
