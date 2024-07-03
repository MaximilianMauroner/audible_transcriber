<script lang="ts">
  import { page } from "$app/stores";
  import { readDir, BaseDirectory } from "@tauri-apps/api/fs";

  let name = $page.params.name;

  let inclsionList = [".mp3", ".m4b", ".m4a", ".flac", ".wav", ".ogg"];

  let audibleFiles: { path: string; name: string }[] = [];
  let metadata = null;

  readDir("assets/" + name, {
    dir: BaseDirectory.AppData,
  }).then((files) => {
    console.log(files);
    const jsonFile = files.find((fi) => fi.name?.endsWith(".json"));
    audibleFiles = files
      .filter((e) => {
        return (
          inclsionList.find((inclsion) => e.name?.includes(inclsion)) !==
          undefined
        );
      })
      .map((fi) => {
        return {
          name: fi.name as string,
          path: fi.path,
        };
      });
  });
</script>

<h1 class="p-2 text-center text-lg font-semibold">{name}</h1>
<div class="flex flex-col items-center justify-center">
  {#each audibleFiles as file}
    <div>{file.name}</div>
  {/each}
</div>
