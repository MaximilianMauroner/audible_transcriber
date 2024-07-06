<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { open } from "@tauri-apps/api/dialog";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";

  let audibleFolders: { cover: string; name: string }[] = [];

  const handleClick = async () => {
    const selected = await open({
      directory: true,
      multiple: true,
    });

    if (Array.isArray(selected)) {
      for (const dir of selected) {
        const result = (await invoke("add_directory", { dirPath: dir })) as {
          success: boolean;
          message: string;
        };
        if (!result.success) {
          console.error(result);
        }
        getDirectoryContents();
      }
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
    } else {
      const result = (await invoke("add_directory", { dirPath: selected })) as {
        success: boolean;
        message: string;
      };
      if (!result.success) {
        console.error(result);
      }
      getDirectoryContents();
    }
  };

  const getDirectoryContents = async () => {
    const folders = await readDir("assets", {
      dir: BaseDirectory.AppData,
      recursive: true,
    });
    audibleFolders = folders
      .map((fi) => {
        const cover = fi.children?.find((e) => e.name?.includes(".jpg"));
        return {
          name: fi.name as string,
          cover: cover ? convertFileSrc(cover.path) : "",
        };
      })
      .sort((a, b) => a.name.localeCompare(b.name));
  };
  getDirectoryContents();
  setInterval(getDirectoryContents, 10000);
</script>

<nav class="flex justify-between">
  <button
    on:click={handleClick}
    class="absolute left-0 top-0 z-50 mr-4 mt-2 -translate-x-2 rounded-lg rounded-l-none bg-indigo-800 px-4 py-2 text-white transition duration-100 ease-in-out hover:translate-x-0 hover:pl-8 hover:shadow-md"
    >Add a directory where one audible title is located</button
  >
</nav>
<div class="flex min-h-screen flex-col items-center justify-center gap-2 px-24">
  <div
    class="mx-2 grid items-center justify-center gap-4 sm:grid-cols-2 lg:grid-cols-3"
  >
    {#each audibleFolders as folder}
      <div
        class="relative rounded-lg transition duration-100 ease-in-out hover:scale-105 hover:shadow-md"
      >
        <button
          class="absolute right-1 top-1 z-50 rounded-full bg-black/50 text-white transition duration-150 ease-in-out hover:rotate-90"
          on:click={async () => {
            console.log();

            if (
              await confirm(
                "Are you sure you want to remove this book, this will remove the save?",
              )
            ) {
              const response = await invoke("remove_directory", {
                folderName: folder.name,
              });
              if (response !== "success") console.error(response);

              getDirectoryContents();
            }
          }}
          ><svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="size-6"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M6 18 18 6M6 6l12 12"
            />
          </svg>
        </button>
        <a href={"/folder/" + folder.name}>
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
      </div>
    {/each}
  </div>
</div>
