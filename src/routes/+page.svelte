<script lang="ts">
  import {invoke} from "@tauri-apps/api/core";
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let name = $state("");
  let greetMsg = $state("");
  ShowWindow()

  async function ShowWindow(){
    console.log("Setting window visibility to true")
    await getCurrentWindow().show();
  }

</script>

<main class="container">
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