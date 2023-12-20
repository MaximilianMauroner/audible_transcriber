<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let files: FileList | null;

    const handleSubmit = (event: Event) => {
        event.preventDefault();

        if (!files || files.length === 0) {
            console.error("No files selected");
            return;
        }

        const file = files[0];
        const reader = new FileReader();

        reader.onload = async (event) => {
            if (!event.target) return;

            const fileContents = event.target.result as ArrayBuffer;
            const chunkSize = 1024 * 1024 * 10; // 10MB
            const numChunks = Math.ceil(fileContents.byteLength / chunkSize);
            for (let i = 0; i < numChunks; i++) {
                const start = i * chunkSize;
                const end = Math.min(
                    start + chunkSize,
                    fileContents.byteLength
                );

                const chunk = fileContents.slice(start, end);
                const res = await invoke("append_chunk_to_file", {
                    path: file.name,
                    chunk: Array.from(new Uint8Array(chunk)),
                });
                console.log(res, Number.isInteger(res));

                if (!res) {
                    console.log("error");
                    return false;
                }
            }
        };

        reader.readAsArrayBuffer(file);
    };
</script>

<main class="h-full min-h-screen p-4">
    <div class="container m-auto pt-2">
        <h1 class="text-4xl text-center text-gray-800">Audible Transcriber</h1>
    </div>
    <form on:submit={handleSubmit} class="max-w-screen-md m-auto">
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
                            <div
                                class="mt-4 flex text-sm leading-6 text-gray-600"
                            >
                                <label
                                    for="file-upload"
                                    class="relative cursor-pointer rounded-md bg-white font-semibold text-indigo-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-indigo-600 focus-within:ring-offset-2 hover:text-indigo-500"
                                >
                                    <span>Upload the .aax file</span>
                                    <input
                                        bind:files
                                        id="file-upload"
                                        name="file-upload"
                                        type="file"
                                        class="sr-only"
                                        required
                                    />
                                </label>
                                <p class="pl-1">
                                    or drag and drop your audible file
                                </p>
                            </div>
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
                type="submit"
                class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >Save</button
            >
        </div>
    </form>
</main>
