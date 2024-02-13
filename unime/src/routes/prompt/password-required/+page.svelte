<script lang="ts">
  import { onMount } from 'svelte';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';
  import UniMeLogoDark from '$src/lib/static/svg/logo/UniMeLogoDark.svelte';
  import UniMeLogoLight from '$src/lib/static/svg/logo/UniMeLogoLight.svelte';
  import { state } from '$src/stores';

  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';

  let showPassword = false;

  let password: string;

  // TODO: make reactive
  const isDarkModeEnabled = document.documentElement.classList.contains('dark');

  onMount(() => {
    if ($state?.dev_mode_enabled && $state?.active_profile?.name === 'Ferris') {
      console.log('Developer mode - Injecting password automatically ...');
      setTimeout(() => {
        dispatch({ type: '[Storage] Unlock', payload: { password: 'sup3rSecr3t' } });
      }, 500);
    }
  });
</script>

<div class="content-height bg-silver dark:bg-navy flex items-center justify-center">
  <!-- Placeholder -->
  <!-- <div class="aspect-square w-1/4 rounded-3xl border border-slate-200 bg-slate-100" /> -->
  <div class="flex flex-col items-center justify-center">
    {#if isDarkModeEnabled}
      <UniMeLogoDark />
    {:else}
      <UniMeLogoLight />
    {/if}
    {#if !($state?.dev_mode_enabled && $state?.active_profile?.name === 'Ferris')}
      <div class="relative mb-4 mt-8 w-[240px]">
        <input
          type={showPassword ? 'text' : 'password'}
          class="dark:bg-dark h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:text-slate-300"
          placeholder={$LL.LOCK_SCREEN.PASSWORD_INPUT_PLACEHOLDER()}
          on:input={(e) => (password = e.target.value)}
        />
        <div class="absolute right-3 top-0 flex h-full items-center">
          <button class="rounded-full p-2" on:click={() => (showPassword = !showPassword)}>
            {#if showPassword}
              <Eye class="dark:text-grey text-slate-700" />
            {:else}
              <EyeClosed class="dark:text-grey text-slate-700" />
            {/if}
          </button>
        </div>
      </div>
      <Button
        label={$LL.LOCK_SCREEN.BUTTON_TEXT()}
        on:click={() => dispatch({ type: '[Storage] Unlock', payload: { password } })}
        disabled={!password}
      />
      <!-- Forgot password? Reset app -->
      <div class="mt-8">
        <BottomDrawer titleText={$LL.SETTINGS.RESET_APP.TITLE()} descriptionText={$LL.SETTINGS.RESET_APP.DESCRIPTION()}>
          <button
            slot="trigger"
            let:trigger
            use:melt={trigger}
            class="active:bg-grey dark:active:bg-dark rounded-xl px-4 py-2 text-[13px]/[24px] font-medium text-slate-400 opacity-50"
            >{$LL.LOCK_SCREEN.FORGOT_PASSWORD()}</button
          >

          <!-- TODO: bug: after resetting (closing the drawer, main UI is not clickable anymore) -->
          <div slot="content" class="w-full pb-[10px] pt-[20px]">
            <button
              class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
              on:click={() => dispatch({ type: '[App] Reset' })}>{$LL.SETTINGS.RESET_APP.CONFIRM()}</button
            >
          </div>

          <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.SETTINGS.RESET_APP.CANCEL()} />
        </BottomDrawer>
      </div>
    {/if}
  </div>
</div>

<!-- Overwrite colors from template -->
<div class="safe-area-bottom bg-silver dark:bg-navy z-10" />

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }

  :global(body) {
    /* Fixes a UI problem on iOS where there is a white bar at the bottom when the emoji drawer is open */
    position: unset !important;
  }
</style>
