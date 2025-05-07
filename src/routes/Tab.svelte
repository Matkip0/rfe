<script lang="ts">
    // TODO: functionality for tab content
    let tabs = ["Tab1", "Tab2", "Tab3"];
    let activeTab = 0;

    let showContextMenu = false;
    let contextMenuX = 0;
    let contextMenuY = 0;
    let rightClickedTabIndex: number | null = null;

    let newTabName = "";
    let showRenamePopup = false;

    let containerEl: HTMLElement;

    function toggleTab(index: number) {
        activeTab = index;
    }

    function deleteTab(index: number) {
        tabs = tabs.filter((_, i) => i !== index);
        if (tabs.length > 0) {
            if (activeTab >= tabs.length) {
                activeTab = tabs.length - 1;
            }
        } else {
            activeTab = 0;
        }
    }

    function closeOtherTabs() {
        if (rightClickedTabIndex !== null) {
            tabs = [tabs[rightClickedTabIndex]];
            activeTab = 0;
        }
        showContextMenu = false;
    }

    function handleRightClick(event: MouseEvent, index: number | null = null) {
        event.preventDefault();
        event.stopPropagation();

        const rect = containerEl.getBoundingClientRect();
        contextMenuX = event.clientX - rect.left;
        contextMenuY = event.clientY - rect.top;

        rightClickedTabIndex = index;
        newTabName = tabs[index!] || "";
        showContextMenu = true;
    }

    function addHomeTab() {
        tabs = [...tabs, "Home"];
        activeTab = tabs.length - 1;
        showContextMenu = false;
    }

    function closeRightClickedTab() {
        if (rightClickedTabIndex !== null) {
            deleteTab(rightClickedTabIndex);
        }
        showContextMenu = false;
    }

    function openRenamePopup() {
        showContextMenu = false;
        showRenamePopup = true;
        setTimeout(() => {
            const inputField = document.querySelector('input[name="text"]') as HTMLInputElement;
            if (inputField) inputField.focus();
        }, 100);
    }

    function renameTab() {
        if (rightClickedTabIndex !== null && newTabName.trim() !== "") {
            tabs[rightClickedTabIndex] = newTabName.trim();
        }
        showRenamePopup = false;
    }

    function cancelRename() {
        showRenamePopup = false;
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            renameTab();
        } else if (event.key === "Escape") {
            cancelRename();
        }
    }

    document.addEventListener("click", () => {
        showContextMenu = false;
    });
</script>

<div
        aria-label="Right-click to open context menu"
        bind:this={containerEl}
        class="relative w-full"
        on:contextmenu={(e) => handleRightClick(e, null)}
        role="button"
        tabindex="0"
>
    {#if tabs.length > 1}
        <div class="overflow-x-auto w-full">
            <div
                    class="grid auto-cols-fr gap-1 px-1 py-2"
                    style="grid-auto-flow: column; min-width: max-content;">
                {#each tabs as tab, index (index)}
                    <div class="relative min-w-[160px]" on:contextmenu={(e) => handleRightClick(e, index)} role="button"
                         tabindex="0">
                        <button
                                type="button"
                                on:click={() => toggleTab(index)}
                                class={`w-full py-3 px-4 pr-8 inline-flex items-center justify-center gap-x-2 text-sm font-bold focus:z-10 border border-rfe-surface0
								${index === 0 ? 'rounded-l-md' : ''}
								${index === tabs.length - 1 ? 'rounded-r-md' : ''}
								${activeTab === index ? 'bg-rfe-blue text-rfe-crust' : 'bg-transparent hover:bg-rfe-surface1 hover:text-rfe-crust'}`}>
                            {tab}
                        </button>
                        <button
                                type="button"
                                on:click={() => deleteTab(index)}
                                class="absolute right-1 top-1/2 -translate-y-1/2 text-lg text-rfe-peach hover:text-rfe-yellow"
                                aria-label="Close">
                            ✕
                        </button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}

    {#if showContextMenu}
        <div
                class="absolute bg-rfe-base border border-rfe-surface0 shadow-md rounded-2xl p-1 z-50"
                style="top: {contextMenuY}px; left: {contextMenuX}px;"
                on:click|stopPropagation
                role="menu"
                aria-label="Context Menu"
                tabindex="0">
            <div class="flex flex-col divide-y divide-rfe-surface0">
                <button class="hover:bg-rfe-surface1 px-2 py-1 cursor-pointer w-full rounded-t-2xl"
                        on:click={addHomeTab}>Add new Tab
                </button>
                {#if rightClickedTabIndex !== null}
                    <button class="hover:bg-rfe-surface1 px-2 py-1 cursor-pointer w-full"
                            on:click={closeRightClickedTab}>Close This Tab
                    </button>
                    <button class="hover:bg-rfe-surface1 px-2 py-1 cursor-pointer w-full" on:click={closeOtherTabs}>
                        Close Other Tabs
                    </button>
                    <button class="hover:bg-rfe-surface1 px-2 py-1 cursor-pointer w-full rounded-b-2xl"
                            on:click={openRenamePopup}>Rename Tab
                    </button>
                {/if}
            </div>
        </div>
    {/if}

    {#if showRenamePopup}
        <div class="fixed inset-0 bg-rfe-crust/60 flex items-center justify-center z-50" on:click={cancelRename}>
            <div class="bg-rfe-base border shadow-lg rounded p-4 w-96" on:click|stopPropagation>
                <h2 class="text-lg font-semibold mb-2">Rename Tab</h2>
                <input
                        bind:value={newTabName}
                        class="input h-[34px] text-[14px] w-full mb-3 text-rfe-text/60 bg-rfe-crust px-3 py-1 rounded-lg border border-white/10 focus:outline-none focus:ring-2 focus:ring-rfe-yellow focus:ring-offset-2 focus:ring-offset-rfe-crust transition-all duration-150 ease-in-out"
                        name="text"
                        type="text"
                        placeholder="Enter new Tab name"
                        on:keydown={handleKeyDown}
                />
                <div class="flex justify-end gap-2">
                    <button on:click={cancelRename} class="px-3 py-1 bg-rfe-surface1 hover:bg-rfe-surface2 rounded">
                        Cancel
                    </button>
                    <button on:click={renameTab}
                            class="px-3 py-1 bg-rfe-blue text-white hover:bg-rfe-blue-dark rounded">Rename
                    </button>
                </div>
            </div>
        </div>
    {/if}
</div>