<script lang="ts">
    interface directory {
        itemCount: number
        path: string
    }

    interface file {
        fileType: string
        filePath: string
    }

    interface item {
        name: string
        size: string
        modified: string
        directory?: directory
        file?: file
        selected: boolean
    }

    interface sorting {
        sortBy: string
        ascending: boolean
    }

    let currentPath:string = ""
    let sorting: sorting = $state({sortBy: "name", ascending: true})

    //Real data not a placeholder FR FR on god no cap
    const items:item[] = $state([
        {name: "test.txt", size: "2mb", modified: "1970-01-01", selected: false, file: {fileType: "test", filePath: "/bla/bla"}},
        {name: "test2.txt", size: "24mb", modified: "1970-01-01", selected: false},
        {name: "test3.txt", size: "1mb", modified: "1970-01-01", selected: false},
        {name: "test4.txt", size: "8mb", modified: "1970-01-01", selected: false},
        {name: "test5.txt", size: "3mb", modified: "1970-01-01", selected: false},
        {name: "test.txt", size: "2mb", modified: "1970-01-01", selected: false},
        {name: "test2.txt", size: "24mb", modified: "1970-01-01", selected: false},
        {name: "test3.txt", size: "1mb", modified: "1970-01-01", selected: false},
        {name: "test4.txt", size: "8mb", modified: "1970-01-01", selected: false},
        {name: "test5.txt", size: "3mb", modified: "1970-01-01", selected: false},
        {name: "test.txt", size: "2mb", modified: "1970-01-01", selected: false},
        {name: "test2.txt", size: "24mb", modified: "1970-01-01", selected: false},
        {name: "test3.txt", size: "1mb", modified: "1970-01-01", selected: false},
        {name: "test4.txt", size: "8mb", modified: "1970-01-01", selected: false},
        {name: "test5.txt", size: "3mb", modified: "1970-01-01", selected: false},
        {name: "test.txt", size: "2mb", modified: "1970-01-01", selected: false},
        {name: "test2.txt", size: "24mb", modified: "1970-01-01", selected: false},
        {name: "test3.txt", size: "1mb", modified: "1970-01-01", selected: false},
        {name: "test4.txt", size: "8mb", modified: "1970-01-01", selected: false},
        {name: "test5.txt", size: "3mb", modified: "1970-01-01", selected: false},
        {name: "test.txt", size: "2mb", modified: "1970-01-01", selected: false},
        {name: "test2.txt", size: "24mb", modified: "1970-01-01", selected: false},
        {name: "test3.txt", size: "1mb", modified: "1970-01-01", selected: false},
        {name: "test4.txt", size: "8mb", modified: "1970-01-01", selected: false},
        {name: "test5.txt", size: "3mb", modified: "1970-01-01", selected: false},
        {name: "test.txt", size: "2mb", modified: "1970-01-01", selected: false},
        {name: "test2.txt", size: "24mb", modified: "1970-01-01", selected: false},
        {name: "test3.txt", size: "1mb", modified: "1970-01-01", selected: false},
        {name: "test4.txt", size: "8mb", modified: "1970-01-01", selected: false},
        {name: "test5.txt", size: "3mb", modified: "1970-01-01", selected: false},
    ])

    function sortBy(value: string) {
        if (sorting.sortBy === value){
            sorting.ascending = !sorting.ascending
        } else {
            sorting.sortBy = value
            sorting.ascending = true
        }
        //TODO: call rust code to get the new sorted list
        console.log(`sorting by ${value}`)
    }

    function itemClicked(item: item) {
        items.forEach((value) => value.selected = false)
        item.selected = true
        console.log($state.snapshot(item))

    }

    function itemDoubleClicked(item: item) {
        if (typeof item.file !== "undefined")
            openFile(item.file)
        if (typeof item.directory !== "undefined")
            openDirectory(item.directory)
    }

    function openFile(file: file) {
        //TODO: call rust code to open the file
    }

    function openDirectory(dir: directory) {

    }

    function updatePath(newPath:string) {
        try {
            //TODO: call rust code the content of the updated path
            currentPath = newPath
        }
        catch (e) {
            //TODO: Error handling
        }
    }

</script>

<div>
    <table class="table table-hover">
        <thead>
        <tr class="bg-rfe-surface2">
            <th class="" onclick={() => sortBy("name")}>
                <div class="flex">
                    <p>Name</p>
                    {#if sorting.sortBy === "name"}
                        {#if sorting.ascending}
                            <p>▲</p>
                        {:else}
                            <p>▼</p>
                        {/if}
                    {/if}
                </div>
            </th>
            <th class="w-32" onclick={() => sortBy("size")}>
                <div class="flex">
                    <p>Size</p>
                    {#if sorting.sortBy === "size"}
                        {#if sorting.ascending}
                            <p>▲</p>
                        {:else}
                            <p>▼</p>
                        {/if}
                    {/if}
                </div>
            </th>
            <th class="w-64" onclick={() => sortBy("modified")}>
                <div class="flex">
                    <p>Modified</p>
                    {#if sorting.sortBy === "modified"}
                        {#if sorting.ascending}
                            <p>▲</p>
                        {:else}
                            <p>▼</p>
                        {/if}
                    {/if}
                </div>
            </th>
        </tr>
        </thead>
        <tbody>
        {#each items as item, name (name)}
            <tr class="clickable"
                class:bg-rfe-blue={item.selected}
                class:even:bg-rfe-mantle={!item.selected}
                ondblclick={() => itemDoubleClicked(item)}
                onclick={() => itemClicked(item)}>

                <td>{item.name}</td>
                <td>{item.size}</td>
                <td>{item.modified}</td>
            </tr>
        {/each}
        </tbody>
    </table>
</div>

<style>
    .clickable {
        cursor: pointer;
    }

    .table {
        border-collapse: collapse;
        text-indent: 10px;
        width: 100%;
        -webkit-user-select: none;
    }

    .table th {
        text-align: left;
        border: 2px solid #9399b2;
    }


    .table th {
        position: relative;
        cursor: grab;
        user-select: none;
    }
    .table th:active {
        cursor: grabbing;
    }

</style>