<script lang="ts">
    import { addNewTab } from './stores/tabs';
    import {getCurrentWindow} from '@tauri-apps/api/window';
    import {onDestroy, onMount} from 'svelte';

    const appWindow = getCurrentWindow();
    let isMaximized = false;
    let showDropdown = false;


    onMount(async () => {
        isMaximized = await appWindow.isMaximized();
    });

    function minimize() {
        appWindow.minimize();
    }

    async function toggleMaximize() {
        appWindow.toggleMaximize();
        isMaximized = await appWindow.isMaximized();
    }

    function close() {
        appWindow.close();
    }

    function goBack() {
        console.log('Going back');
    }

    function goForward() {
        console.log('Going forward');
    }

    function toggleDropdown() {
        showDropdown = !showDropdown;
    }

    function handleDropdownAction(action: string) {
        console.log(`Dropdown action: ${action}`);
        if (action === 'New Tab') {
            addNewTab();
        }
        showDropdown = false;
    }

    // Drop Down Menu
    let dropdownRef: HTMLElement;
    let dropdownButtonRef: HTMLElement;

    function handleClickOutside(event: MouseEvent) {
        if (
            showDropdown &&
            !dropdownRef?.contains(event.target as Node) &&
            !dropdownButtonRef?.contains(event.target as Node)
        ) {
            showDropdown = false;
        }
    }

    function handleEscape(event: KeyboardEvent) {
        if (event.key === 'Escape' && showDropdown) {
            showDropdown = false;
        }
    }

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        document.addEventListener('keydown', handleEscape);
    });

    onDestroy(() => {
        document.removeEventListener('click', handleClickOutside);
        document.removeEventListener('keydown', handleEscape);
    });
</script>

<div>
    <div
            class="h-[30px] select-none flex justify-between items-center px-[10px] relative"
            data-tauri-drag-region
    >
        <div class="flex items-center gap-1">
            <button class="inline-flex justify-center items-center w-[30px] h-[30px] select-none hover:text-white"
                    onclick={goBack}>
                <span class="text-xl">&lt;</span>
            </button>
            <button class="inline-flex justify-center items-center w-[30px] h-[30px] select-none hover:text-white"
                    onclick={goForward}>
                <span class="text-xl">&gt;</span>
            </button>

            <!-- Dropdown -->
            <div class="relative">
                <button bind:this={dropdownButtonRef}
                        class="inline-flex justify-center items-center w-[30px] h-[30px] hover:text-white"
                        onclick={toggleDropdown} >
                    <svg class="w-5 h-5"
                         fill="none"
                         stroke="currentColor"
                         stroke-width="2"
                         viewBox="0 0 24 24"
                         xmlns="http://www.w3.org/2000/svg" >
                        <path d="M4 6h16M4 12h16M4 18h16" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </button>

                {#if showDropdown}
                    <div class="absolute mt-1 w-40 bg-rfe-base border border-rfe-surface0 rounded shadow z-10"
                         bind:this={dropdownRef}>
                        <button class="w-full text-left px-4 py-2 hover:bg-rfe-surface1"
                                onclick={() => handleDropdownAction('New Tab')}>
                            New Tab
                        </button>

                        <div class="h-px bg-rfe-surface0 mx-2 my-1"></div>

                        <button class="w-full text-left px-4 py-2 hover:bg-rfe-surface1"
                                onclick={() => handleDropdownAction('Sort By')}>
                            Sort By
                        </button>

                        <div class="h-px bg-rfe-surface0 mx-2 my-1"></div>

                        <button class="w-full text-left px-4 py-2 hover:bg-rfe-surface1"
                                onclick={() => handleDropdownAction('Copy URL')}>
                            Copy URL
                        </button>

                        <button class="w-full text-left px-4 py-2 hover:bg-rfe-surface1"
                                onclick={() => handleDropdownAction('Preferences')}>
                            Preferences
                        </button>
                    </div>
                {/if}
            </div>
        </div>

        <!-- URL -->
        <div class="absolute left-1/2 -translate-x-1/2 w-full flex justify-center pointer-events-none">
            <input
                    class="pointer-events-auto w-[40%] max-w-[500px] min-w-[200px] h-[24px] px-3 py-[2px] text-white bg-[#222630] border-2 border-[#2B3040] focus:border-[#596A95] rounded-md outline-none transition-colors duration-100"
                    name="text"
                    placeholder="Enter URL"
                    type="text"
            />
        </div>

        <!-- Window Options -->
        <div class="flex items-center gap-1">
            <button class="inline-flex justify-center items-center w-[22px] h-[22px] select-none bg-rfe-surface1 hover:bg-rfe-text rounded-full"
                    onclick={minimize}>
                <img alt="minimize" src="https://api.iconify.design/mdi:window-minimize.svg"/>
            </button>
            <button class="inline-flex justify-center items-center w-[22px] h-[22px] select-none bg-rfe-surface1 hover:bg-rfe-text rounded-full"
                    onclick={toggleMaximize}>
                <img alt={isMaximized ? "restore" : "maximize"}
                     src={isMaximized ? "https://api.iconify.design/mdi:window-restore.svg" : "https://api.iconify.design/mdi:window-maximize.svg"}/>
            </button>
            <button class="inline-flex justify-center items-center w-[22px] h-[22px] select-none bg-rfe-surface1 hover:bg-rfe-text rounded-full"
                    onclick={close}>
                <img alt="close" src="https://api.iconify.design/mdi:close.svg"/>
            </button>
        </div>
    </div>
</div>
