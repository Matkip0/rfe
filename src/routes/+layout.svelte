<script>
    let { children } = $props();
    import "../app.css";
    import Header from './Header.svelte';
</script>

<div class="app">
    <Header />

    <main>
        {@render children()}
    </main>
</div>