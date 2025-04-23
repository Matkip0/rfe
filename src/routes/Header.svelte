<script>
    import { getCurrentWindow } from '@tauri-apps/api/window';

    const appWindow = getCurrentWindow();

    function minimize(){
        console.log("minimize")
        appWindow.minimize()
    }

    function toggleMaximize(){
        console.log("toggleMaximize")
        appWindow.toggleMaximize()
    }

    function close(){
        console.log("close")
        appWindow.close()
    }
</script>

<div>
    <div data-tauri-drag-region class="titlebar">
        <button class="titlebar-button" id="titlebar-minimize" onclick={minimize}>
            <img
                    src="https://api.iconify.design/mdi:window-minimize.svg"
                    alt="minimize"
            />
        </button>
        <button class="titlebar-button" id="titlebar-maximize" onclick={toggleMaximize}>
            <img
                    src="https://api.iconify.design/mdi:window-maximize.svg"
                    alt="maximize"
            />
        </button>
        <button class="titlebar-button" id="titlebar-close" onclick={close}>
            <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
        </button>
    </div>
</div>

<style>
    .titlebar {
        height: 30px;
        background: #525252;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        /*position: fixed;*/
        top: 0;
        left: 0;
        right: 0;
    }
    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 30px;
        height: 30px;
        user-select: none;
        -webkit-user-select: none;
    }
    .titlebar-button:hover {
        background: #ffffff;
    }
</style>