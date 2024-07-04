<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { open } from "@tauri-apps/api/dialog";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";

  let audibleFolders: { cover: string; name: string }[] = [];

  const handleClick = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (Array.isArray(selected)) {
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
    } else {
      const result = await invoke("add_directory", { dirPath: selected });
      console.log(result);
    }
  };

  const getDirectoryContents = async () => {
    const folders = await readDir("assets", {
      dir: BaseDirectory.AppData,
      recursive: true,
    });
    audibleFolders = folders.map((fi) => {
      const cover = fi.children?.find((e) => e.name?.includes(".jpg"));
      return {
        name: fi.name as string,
        cover: cover ? convertFileSrc(cover.path) : "",
      };
    });
  };
  getDirectoryContents();
  setInterval(getDirectoryContents, 10000);
</script>

<nav class="flex justify-between">
  <button
    on:click={handleClick}
    class="absolute left-0 top-0 mr-4 mt-2 -translate-x-2 rounded-lg rounded-l-none bg-indigo-800 px-4 py-2 text-white transition duration-100 ease-in-out hover:translate-x-0 hover:pl-8 hover:shadow-md"
    >Add a directory where one audible title is located</button
  >
</nav>
<div class="flex h-screen flex-col items-center justify-center gap-2">
  <div class="grid grid-cols-2 items-center justify-center gap-2">
    {#each audibleFolders as folder}
      <a
        href={"/folder/" + folder.name}
        class="transition duration-100 ease-in-out hover:scale-105 hover:shadow-md"
      >
        <div class="mx-auto grid max-w-4xl grid-cols-1">
          <div
            class="relative col-start-1 row-start-1 flex w-64 flex-col-reverse rounded-lg bg-gradient-to-t from-black/85 via-black/50 p-3"
          >
            <h1 class="dark mt-1 text-lg font-semibold text-white">
              {folder.name}
            </h1>
          </div>
          <div class="col-start-1 col-end-3 row-start-1 grid gap-4">
            <img
              src={folder.cover}
              alt={folder.name}
              class="h-40 w-full rounded-lg object-cover object-top"
              loading="lazy"
            />
          </div>
        </div>
      </a>
    {/each}
  </div>
</div>
