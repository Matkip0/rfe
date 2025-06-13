<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    interface Directory {
        itemCount: number
    }

    interface File {
        fileType: string
    }

    interface Item {
        name: string
        path: string
        size_formated: string
        modified_formated: string
        directory?: Directory
        file?: File
        selected: boolean
    }

    interface Sorting {
        sortBy: string
        ascending: boolean
    }

    let currentPath: string = $state("/")
    let sorting: Sorting = $state({sortBy: "name", ascending: true})

    let items:Item[] = $state([])

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

    function itemClicked(item: Item) {
        items.forEach((value) => value.selected = false)
        item.selected = true
        console.log($state.snapshot(item))

    }

    function itemDoubleClicked(item: Item) {
        currentPath += item.name + "/"
        if (typeof item.file !== "undefined")
            openFile(item.file)
        if (typeof item.directory !== "undefined")
            openDirectory(item.directory)
        readDirectory()
    }

    function openFile(file: File) {
        //TODO: call rust code to open the file
    }

    function openDirectory(dir: Directory) {

    }

    async function updatePath(newPath:string) {
        try {
            //TODO: get data other then just names when you call rust code
            currentPath = newPath
            await readDirectory()
        }
        catch (e) {
            //TODO: Error handling
        }
    }

    async function readDirectory() {
        let item_names: any
        item_names = await invoke("read_directory", { currentPath });
        //items = item_names
        //console.log($state.snapshot(items))
        console.log(item_names)
        items = [];
        item_names.forEach((value:any) => {
            items.push({
                name: value.name,
                path: value.path,
                modified_formated: value.modified_formated,
                size_formated: value.size_formated,
                selected: false,
                file: readFile(value.item_type.File),
                directory: readDir(value.item_type.Directory),
            })
        })
        console.log(items)
    }
    function readFile(valueItetmType: any)  {
        if (typeof valueItetmType !== "undefined"){
            let file : File
             file = {
                fileType: valueItetmType.file_type
            }
            return file
        } else{
            return undefined
        }
    }
    function readDir(valueItetmType: any)  {
        console.log(valueItetmType)
        if (typeof valueItetmType !== "undefined"){
            let dir : Directory
            dir = {
                itemCount: valueItetmType.item_count
            }
            return dir
        } else{
            return undefined
        }
    }

    readDirectory()
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
                <td>{item.size_formated}</td>
                <td>{item.modified_formated}</td>
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