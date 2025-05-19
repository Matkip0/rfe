import { writable } from 'svelte/store';

export const tabs = writable<string[]>(["Tab1", "Tab2", "Tab3"]);
export const activeTab = writable<number>(0);

export function addNewTab(name: string = "Home") {
    tabs.update(currentTabs => {
        const updated = [...currentTabs, name];
        activeTab.set(updated.length - 1);
        return updated;
    });
}

export function deleteTab(index: number) {
    tabs.update(currentTabs => {
        const updated = currentTabs.filter((_, i) => i !== index);
        activeTab.update(active => {
            if (updated.length === 0) return 0;
            if (active >= updated.length) return updated.length - 1;
            return active;
        });
        return updated;
    });
}

export function setActiveTab(index: number) {
    activeTab.set(index);
}
