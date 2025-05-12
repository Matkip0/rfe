<script>
    import {getCurrentWindow} from '@tauri-apps/api/window';
    import {onMount} from 'svelte';

    const appWindow = getCurrentWindow();
    let isMaximized = false;

    // TODO: Needs to be tested if user change window size by not using buttons
    onMount(async () => {
        isMaximized = await appWindow.isMaximized();
    });

    function minimize() {
        appWindow.minimize();
    }

    // TODO: Needs to be tested if working correctly
    async function toggleMaximize() {
        if (isMaximized) {
            appWindow.unmaximize();
        } else {
            appWindow.maximize();
        }
        isMaximized = !isMaximized;
    }

    function close() {
        appWindow.close();
    }

    // TODO: Add functionality
    function goBack() {
        console.log('Going back');
    }
    function goForward() {
        console.log('Going forward');
    }
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
        </div>

        <div
                class="absolute left-1/2 -translate-x-1/2 w-full flex justify-center pointer-events-none"
        >
            <input
                    class="pointer-events-auto w-[40%] max-w-[500px] min-w-[200px] h-[24px] px-3 py-[2px] text-white bg-[#222630] border-2 border-[#2B3040] focus:border-[#596A95] rounded-md outline-none transition-colors duration-100"
                    name="text"
                    placeholder="Enter URL"
                    type="text"
            />
        </div>

        <div class="flex items-center gap-1">
            <button class="inline-flex justify-center items-center w-[30px] h-[30px] select-none bg-rfe-surface1 hover:bg-rfe-text rounded-full"
                    onclick={minimize}>
                <img alt="minimize" src="https://api.iconify.design/mdi:window-minimize.svg"/>
            </button>
            <button class="inline-flex justify-center items-center w-[30px] h-[30px] select-none bg-rfe-surface1 hover:bg-rfe-text rounded-full"
                    onclick={toggleMaximize}>
                <img alt={isMaximized ? "restore" : "maximize"}
                     src={isMaximized ? "https://api.iconify.design/mdi:window-restore.svg" : "https://api.iconify.design/mdi:window-maximize.svg"}/>
            </button>
            <button class="inline-flex justify-center items-center w-[30px] h-[30px] select-none bg-rfe-surface1 hover:bg-rfe-text rounded-full"
                    onclick={close}>
                <img alt="close" src="https://api.iconify.design/mdi:close.svg"/>
            </button>
        </div>
    </div>
</div>
