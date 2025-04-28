<script>
    let { children } = $props();
    import "../app.css";
    import Header from './Header.svelte';
    import Sidebar from './Sidebar.svelte';
    import Tab from './Tab.svelte';
</script>

<div class="app flex flex-col h-screen">
    <Header />
    <div class="flex flex-1">
        <div class="w-72 bg-gray-700 p-4">
            <Sidebar />
        </div>
        <div class="flex-1 flex flex-col">
            <Tab />
            <main class="flex-1 p-6 bg-gray-900 text-white">
                {@render children()}
            </main>
        </div>
    </div>
</div>