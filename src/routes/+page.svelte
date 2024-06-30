<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import { open } from "@tauri-apps/api/dialog";
  // Open a selection dialog for image files

  let responose = "";
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
      responose = await invoke("load_dir", { dirPath: selected });
      // user selected a single file
    }
  };
</script>

<div class="h-screen flex items-center justify-center">
  <div>
    <button
      class="bg-blue-500 rounded-lg border-1 border-indigo-500 p-2 text-white"
      on:click={handleClick}
      >Select a directory where all your audible folders are located</button
    >
    <p>{responose}</p>
  </div>
</div>
