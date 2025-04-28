<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  ShowWindow()

  async function ShowWindow(){
    console.log("Setting window visibility to true")
    await getCurrentWindow().show();
  }
</script>

<main>
  <h1 class="text-3xl font-bold underline">
    Hello world!
  </h1>
</main>

<style lang="postcss">
  @reference "tailwindcss";
  :global(html) {
    background-color: theme(--color-gray-100);
  }
</style>