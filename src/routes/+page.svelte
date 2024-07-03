<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { open } from "@tauri-apps/api/dialog";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";

  let audibleFolders: { path: string; name: string }[] = [];

  const handleClick = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    console.log(selected);

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
    });
    audibleFolders = folders.map((fi) => {
      return {
        name: fi.name as string,
        path: convertFileSrc(fi.path),
      };
    });
    console.log(audibleFolders);
  };
  getDirectoryContents();
</script>

<div class="row fixed top-0 flex items-center justify-center">
  <div>
    <button
      class="m-1 rounded-lg border border-indigo-500 p-1 text-sm text-black hover:bg-indigo-500 hover:text-white"
      on:click={handleClick}
      >Add a directory where one audible title is located</button
    >
  </div>
</div>
<div class="flex h-screen flex-col items-center justify-center gap-2">
  <div class="flex flex-col items-center justify-center gap-2">
    {#each audibleFolders as folder}
      <a href={"/folder/" + folder.name}>
        <div class="rounded-lg border border-indigo-500 p-4">
          <span>{folder.name}</span>
        </div>
      </a>
    {/each}
  </div>
</div>
