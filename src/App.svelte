<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    const pickerOpts = {
        types: [
            {
                description: "Audible File",
                accept: {
                    "*": [".aax"],
                },
            },
        ],
        excludeAcceptAllOption: true,
        multiple: false,
    };
    let uploading: boolean = false;
    let progress: number = 0;

    const openFileDialog = async () => {
        const currentDirHandle = await window.showDirectoryPicker();
        await returnPathDirectories(currentDirHandle);
    };
    async function returnPathDirectories(
        directoryHandle: FileSystemDirectoryHandle
    ) {
        // Get a file handle by showing a file picker:
        const [handle] = await self.showOpenFilePicker();
        if (!handle) {
            // User cancelled, or otherwise failed to open a file.
            return;
        }

        // Check if handle exists inside our directory handle
        const relativePaths = await directoryHandle.resolve(handle);

        if (relativePaths === null) {
            // Not inside directory handle
        } else {
            // relativePath is an array of names, giving the relative path

            for (const name of relativePaths) {
                // log each entry
                console.log(name);
            }
        }
    }
</script>

<main class="h-full min-h-screen p-4">
    <div class="container m-auto pt-2">
        <h1 class="text-4xl text-center text-gray-800">Audible Transcriber</h1>
    </div>
    {#if uploading}
        <div class="w-full bg-gray-200 rounded-full dark:bg-gray-700">
            <div
                class="bg-blue-600 text-xs font-medium text-blue-100 text-center p-0.5 leading-none rounded-full"
                style={`width: ${progress}%`}
            >
                {progress}%
            </div>
        </div>

        <progress class="w-full" value={progress} max="100"></progress>
    {/if}
    <div class="max-w-screen-md m-auto">
        <div class="space-y-12">
            <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                <div class="col-span-full">
                    <label
                        for="audible-file"
                        class="block text-sm font-medium leading-6 text-gray-900"
                        >Audible File</label
                    >
                    <div
                        class="mt-2 flex justify-center rounded-lg border border-dashed border-gray-900/25 px-6 py-10"
                    >
                        <div class="text-center">
                            <svg
                                class="mx-auto h-12 w-12 text-gray-300"
                                viewBox="0 0 24 24"
                                fill="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M1.5 6a2.25 2.25 0 012.25-2.25h16.5A2.25 2.25 0 0122.5 6v12a2.25 2.25 0 01-2.25 2.25H3.75A2.25 2.25 0 011.5 18V6zM3 16.06V18c0 .414.336.75.75.75h16.5A.75.75 0 0021 18v-1.94l-2.69-2.689a1.5 1.5 0 00-2.12 0l-.88.879.97.97a.75.75 0 11-1.06 1.06l-5.16-5.159a1.5 1.5 0 00-2.12 0L3 16.061zm10.125-7.81a1.125 1.125 0 112.25 0 1.125 1.125 0 01-2.25 0z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                            <button
                                on:click={openFileDialog}
                                class="mt-4 flex text-sm leading-6 text-gray-600"
                            >
                                <label
                                    for="file-upload"
                                    class="relative cursor-pointer rounded-md bg-white font-semibold text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 hover:text-indigo-500"
                                >
                                    <span>Upload the .aax file</span>
                                </label>
                                <p class="pl-1">
                                    or drag and drop your audible file
                                </p>
                            </button>
                            <p class="text-xs leading-5 text-gray-600">.aax</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div class="mt-6 flex items-center justify-end gap-x-6">
            <button
                type="reset"
                class="text-sm font-semibold leading-6 text-gray-900"
                >Cancel</button
            >
            <button
                type="button"
                class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >Save</button
            >
        </div>
    </div>
</main>
