<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { open } from "@tauri-apps/api/dialog";
  // Open a selection dialog for image files
  let audibleDirectorySelected = false;
  let directoryName = "";
  const loadDir = async () => {
    invoke("has_audible_directory").then((res) => {
      if (res) {
        const ret = res as { path: string };
        audibleDirectorySelected = ret.path !== "";
        directoryName = ret.path;
      }
    });
  };
  loadDir();
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
      await invoke("load_dir", { dirPath: selected });
      // user selected a single file
      loadDir();
    }
  };

  const getDirectoryContents = async () => {};
</script>

{#if !audibleDirectorySelected}
  <div class="h-screen flex items-center justify-center">
    <div>
      <button
        class="bg-blue-500 rounded-lg border-1 border-indigo-500 p-2 text-white"
        on:click={handleClick}
        >Select a directory where all your audible folders are located</button
      >
    </div>
  </div>
{/if}

{#if audibleDirectorySelected}
  <div class="h-screen flex justify-center">
    <div>
      <h1 class="text-2xl">Audible directory selected</h1>
      <i>{directoryName}</i>
    </div>
    <div>
      <h1>Audio</h1>
      <audio
        src="file:///C:/Users/maxim/Documents/Audible/Solve%20for%20Happy%20[B06XRZLBJC]/Solve%20for%20Happy%EA%9E%89%20Engineer%20Your%20Path%20to%20Joy%20[B06XRZLBJC]%20-%2001%20-%20Chapter%201.m4b"
      >
      </audio>
    </div>
  </div>
{/if}
