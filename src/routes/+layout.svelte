<script>
    let { children } = $props();
    import "../app.css";
    import Header from './Header.svelte';
    import Sidebar from './Sidebar.svelte';
    import Tab from './Tab.svelte';
</script>

<div class="text-rfe-text surface0 app flex flex-col h-screen overflow-x-hidden">
    <div class="border-b-2 bg-rfe-mantle border-rfe-surface0">
        <Header />
    </div>
    <div class="bg-rfe-mantle flex flex-1 overflow-hidden">
        <div class="w-56 p-4 border-r-2 border-rfe-surface0">
            <Sidebar />
        </div>
        <div class="flex-1 flex flex-col overflow-hidden">
            <div class="bg-rfe-base border-b-2 border-rfe-surface0">
                <Tab />
            </div>
            <main class="bg-rfe-base flex-1 p-6 overflow-auto">
                {@render children()}
            </main>
        </div>
    </div>
</div>
