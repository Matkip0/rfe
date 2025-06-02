<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {onDestroy, onMount} from "svelte";

    let devices: {
        name: string;
        progress: number;
        color: string;
        max_space: string;
        used_space: string;
    }[] = [];

    let intervalId: number;

    async function refreshDevices() {
        try {
            const newDevices = await invoke<typeof devices>("get_devices");
            console.log("Fetched devices:", newDevices);
            devices = [...newDevices];
        } catch (error) {
            console.error("Failed to get devices:", error);
        }
    }

    onMount(() => {
        refreshDevices();

        console.log("Refresh devices");
        intervalId = setInterval(refreshDevices, 3000);

        onDestroy(() => {
            clearInterval(intervalId);
        });
    });
</script>

<div class="no-select w-full flex flex-col rounded-lg overflow-hidden">
    <div class="mb-2">
        <h2 class="subtext0 text-sm font-extrabold mb-2">Places</h2>
        <button class="w-full py-2 px-0 text-left text-sm font-bold bg-transparent flex items-center gap-x-2 cursor-pointer"
                type="button">
            <img alt="Home Icon" class="h-5 w-5" src="./images/icons/house.svg"/>
            Home
        </button>
        <button class="w-full py-2 px-0 text-left text-sm font-bold bg-transparent flex items-center gap-x-2 cursor-pointer"
                type="button">
            <img alt="Home Icon" class="h-5 w-5" src="./images/icons/recent.svg"/>
            Recent
        </button>
        <button class="w-full py-2 px-0 text-left text-sm font-medium bg-transparent flex items-center gap-x-2 cursor-pointer"
                type="button">
            <img alt="Starred Icon" class="h-5 w-5" src="./images/icons/star.svg"/>
            Starred
        </button>
        <button class="w-full py-2 px-0 text-left text-sm font-medium bg-transparent flex items-center gap-x-2 cursor-pointer"
                type="button">
            <img alt="Trash Icon" class="h-5 w-5" src="./images/icons/trash.svg"/>
            Trash
        </button>
    </div>

    <div class="mt-4"></div>

    <div class="mb-2">
        <h2 class="subtext0 text-sm font-extrabold mb-2">Remote</h2>
        <button class="w-full py-2 px-0 text-left text-sm font-medium bg-transparent hover:bg-gray-100 flex items-center gap-x-2 cursor-pointer"
                type="button">
            <img alt="Network Icon" class="h-5 w-5" src="./images/icons/network.svg"/>
            Network
        </button>
    </div>

    <div class="mt-4"></div>

    <div class="no-select w-full flex flex-col rounded-lg">
        <div class="mb-2">
            <h2 class="subtext0 text-sm font-extrabold mb-2">Devices</h2>

            <div class="container">
                {#each devices as device}
                    <div class="relative group hover:bg-[rgba(0,0,0,0.3)] px-2 py-2 rounded-md transition-colors duration-200 cursor-pointer">
                        <h2 class="text-sm font-extrabold mb-2 truncate w-full">
                            {device.max_space} - {device.name}
                        </h2>
                        <div class="progress2 mb-3">
                            <div
                                    class="progress-bar2 {device.color}"
                                    style="width: {device.progress}%"
                            ></div>
                        </div>

                        <div
                                class="absolute top-0 left-0 right-0 translate-y-[-100%] text-xs bg-rfe-base bg-opacity-80 rounded border-1 px-4 py-1 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none device-tooltip"
                        >
                            {#if device.name}
                                <div class="font-semibold mb-1 break-words">Device: {device.name} </div>
                            {/if}
                            <div class="break-words">Used: {device.used_space} / {device.max_space}</div>
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    </div>
</div>

<style>
    button:hover {
        background-color: rgba(0, 0, 0, 0.30);
    }

    .no-select {
        -webkit-user-select: none;
    }

    .container {
        margin: auto;
        width: 100%;
    }

    .progress2 {
        padding: 6px;
        border-radius: 30px;
        background: rgba(0, 0, 0, 0.25);
        box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.25), 0 1px rgba(255, 255, 255, 0.08);
    }

    .progress-bar2 {
        height: 8px;
        border-radius: 30px;
        background-image: linear-gradient(to bottom, rgba(255, 255, 255, 0.3), rgba(255, 255, 255, 0.05));
        transition: 0.4s linear;
        transition-property: width, background-color;
    }
</style>