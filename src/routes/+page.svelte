<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Columns from "./rfe-directory/Columns.svelte";

  ShowWindow()

  async function ShowWindow(){
    console.log("Setting window visibility to true")
    await getCurrentWindow().show();
  }
</script>

<main>
  <Columns/>
</main>

<style lang="postcss">
  @reference "tailwindcss";
  :global(html) {
    background-color: theme(--color-gray-100);
  }
</style>