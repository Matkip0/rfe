<script>
    const devices = [
        { name: '999 GiB internal drive', progress: 43 },
        { name: '512 GiB external drive', progress: 90 },
        { name: '1 TB cloud storage', progress: 25 }
    ];

    devices.forEach(device => {
        device.color = device.progress >= 90 ? 'red' : 'blue';
    });
</script>


<div class="no-select w-full flex flex-col rounded-lg overflow-hidden">
    <div class="mb-2">
        <h2 class="subtext0 text-sm font-extrabold mb-2">Places</h2>
        <button type="button" class="w-full py-2 px-0 text-left text-sm font-bold bg-transparent flex items-center gap-x-2">
            <img src="./images/icons/house.svg" alt="Home Icon" class="h-5 w-5" />
            Home
        </button>
        <button type="button" class="w-full py-2 px-0 text-left text-sm font-bold bg-transparent flex items-center gap-x-2">
            <img src="./images/icons/recent.svg" alt="Home Icon" class="h-5 w-5" />
            Recent
        </button>
        <button type="button" class="w-full py-2 px-0 text-left text-sm font-medium bg-transparent flex items-center gap-x-2">
            <img src="./images/icons/star.svg" alt="Starred Icon" class="h-5 w-5" />
            Starred
        </button>
        <button type="button" class="w-full py-2 px-0 text-left text-sm font-medium bg-transparent flex items-center gap-x-2">
            <img src="./images/icons/trash.svg" alt="Trash Icon" class="h-5 w-5" />
            Trash
        </button>
    </div>

    <div class="mt-4"></div>

    <div class="mb-2">
        <h2 class="subtext0 text-sm font-extrabold mb-2">Remote</h2>
        <button type="button" class="w-full py-2 px-0 text-left text-sm font-medium bg-transparent hover:bg-gray-100 flex items-center gap-x-2">
            <img src="./images/icons/network.svg" alt="Network Icon" class="h-5 w-5" />
            Network
        </button>
    </div>

    <div class="mt-4"></div>

    <div class="no-select w-full flex flex-col rounded-lg overflow-hidden">
        <div class="mb-2">
            <h2 class="subtext0 text-sm font-extrabold mb-2">Devices</h2>

            <div class="container">
                {#each devices as device}
                    <div class="device">
                        <h2 class="text-c text-sm font-extrabold mb-2">{device.name}</h2>
                        <div class="progress2 mb-3">
                            <div class="progress-bar2 bg-rfe-{device.color}" style="width: {device.progress}%;"></div>
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
