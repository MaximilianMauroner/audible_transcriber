<script lang="ts">
  import { page } from "$app/stores";
  import {
    readDir,
    BaseDirectory,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import type { AudioMetadata, Record } from "../../../types";

  let name = $page.params.name;

  let inclsionList = [".mp3", ".m4b", ".m4a", ".flac", ".wav", ".ogg"];

  let audibleFile: { path: string; name: string } | undefined = undefined;
  let metadata: AudioMetadata | undefined;

  let audio = new Audio();
  let isPlaying = false;
  let totalTrackTime: number;
  let totalTimeDisplay = "loading...";
  let currTimeDisplay = "0:00:00";
  let progress = 0;
  let trackTimer: number;
  let isReady = -2;
  let selectedBookmark: number;
  let bookmarkValue: string;
  let playbackRate = 2;
  let saving = false;
  let copied = false;
  let inputEl: HTMLTextAreaElement | null = null;

  const HMSToSeconds = (hms: string) => {
    const a = hms.split(":");
    return +a[0] * 60 * 60 + +a[1] * 60 + +a[2];
  };

  audio.onloadedmetadata = () => {
    totalTrackTime = audio.duration;
    audio.playbackRate = playbackRate;

    metadata?.records.forEach((record) => {
      record.relativePosition = calculateProgress(HMSToSeconds(record.Start));
      record.hidden = record.hidden ?? false;
    });
    metadata?.records.sort((a, b) => a.relativePosition - b.relativePosition);

    isReady++;
    if (metadata?.records.length && metadata?.records.length > 0) {
      selectedBookmark = 0;
      bookmarkValue = metadata?.records[selectedBookmark].Text ?? "";
      setTimeout(() => {
        inputEl?.focus();
      }, 100);
    }
    updateTime();
  };

  const calculateProgress = (calculateChapterPosition = -1) => {
    let curr = audio.currentTime;
    if (calculateChapterPosition > 0) {
      curr = calculateChapterPosition;
    }
    return (curr / audio.duration) * 100;
  };

  const prettyPrintTime = (time: number) => {
    let hours = Math.floor(time / 3600).toString();
    if (+hours < 10) hours = "0" + hours;
    let minutes = Math.floor((time % 3600) / 60).toString();
    if (+minutes < 10) minutes = "0" + minutes;
    let seconds = Math.floor(time % 60).toString();
    if (+seconds < 10) seconds = "0" + seconds;
    return `${hours}:${minutes}:${seconds}`;
  };

  $: audio.playbackRate = playbackRate;

  readDir("assets/" + name, {
    dir: BaseDirectory.AppData,
  }).then(async (files) => {
    const jsonFile = files.find(
      (fi) => fi.name?.endsWith(".json") && !fi.name?.startsWith("save"),
    );
    const saveFile = files.find((fi) => fi.name?.startsWith("save"));
    if (jsonFile === undefined) {
      console.error("No metadata file found");
      return;
    }

    const text = await readTextFile(jsonFile?.path as string);
    metadata = JSON.parse(text) as AudioMetadata;

    const timestampMap = new Map<string, number>();
    metadata.records = metadata.records
      .filter((e) => {
        if (timestampMap.has(e.Start)) return false;
        timestampMap.set(e.Start, 1);
        return true;
      })
      .map((record) => {
        record.hidden = record.hidden ?? false;
        return record;
      });

    if (saveFile) {
      const saveText = await readTextFile(saveFile?.path as string);
      const saveData = JSON.parse(saveText) as Record[];
      metadata.records = saveData;
    }

    const f = files.find((e) => {
      return (
        inclsionList.find((inclsion) => e.name?.includes(inclsion)) !==
        undefined
      );
    });
    if (!f || !f.name) return;
    audibleFile = {
      name: f.name,
      path: convertFileSrc(f.path),
    };

    audio.src = audibleFile.path;
    isReady++;
  });

  function updateDisplayTime() {
    currTimeDisplay = prettyPrintTime(audio.currentTime);
    totalTimeDisplay = prettyPrintTime(audio.duration);

    if (audio.ended) {
      toggleTimeRunning();
    }
  }
  const toggleTimeRunning = () => {
    if (audio.ended) {
      isPlaying = false;
      clearInterval(trackTimer);
      console.log(`Ended = ${audio.ended}`);
    } else {
      trackTimer = setInterval(updateTime, 10);
    }
  };

  function updateTime() {
    progress = calculateProgress();
    updateDisplayTime();
  }

  const skipRate = 10;

  const rewindAudio = () => {
    if (audio.currentTime < skipRate) {
      audio.currentTime = 0;
    } else {
      audio.currentTime -= skipRate;
    }
    updateTime();
  };
  const forwardAudio = () => {
    if (audio.currentTime + skipRate > audio.duration) {
      audio.currentTime = audio.duration;
    } else {
      audio.currentTime += skipRate;
    }
    updateTime();
  };

  const playPauseAudio = () => {
    if (audio.paused) {
      toggleTimeRunning();
      audio.play();
      isPlaying = true;
    } else {
      toggleTimeRunning();
      audio.pause();
      isPlaying = false;
    }
  };
  const getCurrentTimeByPercentage = (percentage: number) => {
    return (percentage / 100) * audio.duration;
  };
  const saveData = () => {
    if (metadata) {
      saving = true;
      writeTextFile(
        "assets/" + name + "/save.json",
        JSON.stringify(metadata.records),
        {
          dir: BaseDirectory.AppData,
        },
      ).then(() => {
        setTimeout(() => {
          saving = false;
        }, 250);
      });
    }
  };

  const getActiveClasses = (
    record: Record,
    selectedBookmark: number,
    index: number,
  ) => {
    let val =
      "flex items-center gap-3 rounded-lg border border-gray-200 px-2 py-1";
    if (selectedBookmark === index && !record.hidden) {
      val += selectedBookmark === index ? " bg-indigo-700 text-white" : "";
    } else if (
      record.hidden ||
      (record.Text !== undefined && record.Text !== "" && record.Text !== null)
    ) {
      val += " order-last";
    }
    return val;
  };

  const changeBookmark = (idx: number) => {
    const record = metadata?.records[idx];
    if (!record) return;
    audio.currentTime = HMSToSeconds(record.Start);
    selectedBookmark = idx;
    bookmarkValue = record.Text ?? "";
    inputEl?.focus();
    updateTime();
  };
</script>

<nav class="flex justify-between">
  <a
    tabindex="-1"
    href="/"
    on:click={() => {
      if (isPlaying) {
        playPauseAudio();
      }
    }}
    class="absolute left-0 top-0 mr-4 mt-2 -translate-x-2 rounded-lg rounded-l-none bg-indigo-800 px-4 py-2 text-white transition duration-100 ease-in-out hover:translate-x-0 hover:pl-8 hover:shadow-md"
    >Back</a
  >
  <div></div>
  <h1 class="p-2 text-center text-lg font-semibold">{name}</h1>
  <h3 class="p-2 text-center">
    Bookmarks: {metadata?.records.length || "loading"}
  </h3>
</nav>
{#if saving}
  <div
    class="text-muted-foreground text-sm2 absolute bottom-2 right-2 flex items-center gap-2 rounded-lg bg-indigo-500 p-2 text-white"
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-4 w-4 animate-spin"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M12 2v4" />
      <path d="m16.2 7.8 2.9-2.9" />
      <path d="M18 12h4" />
      <path d="m16.2 16.2 2.9 2.9" />
      <path d="M12 18v4" />
      <path d="m4.9 19.1 2.9-2.9" />
      <path d="M2 12h4" />
      <path d="m4.9 4.9 2.9 2.9" />
    </svg>
    <span class="text-primary-foreground text-sm">Saving</span>
  </div>
{/if}
{#if copied}
  <div
    class="text-muted-foreground text-sm2 absolute bottom-2 left-2 flex items-center gap-2 rounded-lg bg-indigo-500 p-2 text-white"
  >
    <span>Copied</span>
  </div>
{/if}

<div class="mx-auto flex w-full flex-col items-center justify-center px-4">
  {#if audibleFile}
    <div class="flex h-20 w-full items-center justify-center px-4">
      <button
        tabindex="-1"
        on:click={rewindAudio}
        class="group h-8 w-8 rounded-full p-0 focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-700"
      >
        <svg
          class="h-7 w-7 fill-none stroke-indigo-500 group-hover:stroke-indigo-700"
          id="backward"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path
            d="M8 5L5 8M5 8L8 11M5 8H13.5C16.5376 8 19 10.4624 19 13.5C19 15.4826 18.148 17.2202 17 18.188"
          ></path>
          <path d="M5 15V19"></path>
          <path
            d="M8 18V16C8 15.4477 8.44772 15 9 15H10C10.5523 15 11 15.4477 11 16V18C11 18.5523
                10.5523 19 10 19H9C8.44772 19 8 18.5523 8 18Z"
          ></path>
        </svg>
      </button>
      <button
        tabindex="-1"
        on:click={playPauseAudio}
        class="mx-3 h-10 w-10 rounded-full bg-indigo-700 p-2 hover:bg-indigo-900 focus:outline-none focus:ring-2 focus:ring-indigo-700 focus:ring-offset-2"
      >
        {#if !isPlaying}
          <svg aria-hidden="true" class="relative left-px" viewBox="0 0 24 24">
            <path
              fill-rule="evenodd"
              class="fill-current text-white"
              d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0
          3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z"
              clip-rule="evenodd"
            ></path>
          </svg>
        {:else}
          <svg aria-hidden="true" class="relative left-px" viewBox="0 0 24 24">
            <path
              fill-rule="evenodd"
              class="fill-current text-white"
              d="M6.75 5.25a.75.75 0 01.75-.75H9a.75.75 0 01.75.75v13.5a.75.75 0
        01-.75.75H7.5a.75.75 0 01-.75-.75V5.25zm7.5 0A.75.75 0 0115 4.5h1.5a.75.75 0 01.75.75v13.5a.75.75 0
        01-.75.75H15a.75.75 0 01-.75-.75V5.25z"
              clip-rule="evenodd"
            ></path>
          </svg>
        {/if}
      </button>
      <button
        tabindex="-1"
        on:click={forwardAudio}
        class="group relative h-8 w-8 rounded-full p-0 focus:outline-none focus-visible:ring-2 focus-visible:ring-indigo-700"
      >
        <svg
          aria-hidden="true"
          class="h-7 w-7 fill-none stroke-indigo-500 group-hover:stroke-indigo-700"
          viewBox="0 0 24 24"
        >
          <path
            d="M16 5L19 8M19 8L16 11M19 8H10.5C7.46243 8 5 10.4624 5 13.5C5 15.4826 5.85204 17.2202 7 18.188"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></path>
          <path
            d="M13 15V19"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></path>
          <path
            d="M16 18V16C16 15.4477 16.4477 15 17 15H18C18.5523 15 19 15.4477 19 16V18C19 18.5523 18.5523 19 18
          19H17C16.4477 19 16 18.5523 16 18Z"
            stroke-width="1.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></path>
        </svg>
      </button>
      <input
        tabindex="-1"
        type="range"
        min="0.5"
        max="3"
        step="0.25"
        class="ml-4 h-1 w-64 cursor-pointer appearance-none rounded-full bg-indigo-500"
        bind:value={playbackRate}
      />
      <span class="w-8 px-2 text-indigo-800">{playbackRate}x</span>
    </div>
    <div class="flex h-20 w-full items-center pb-3">
      <span class="px-2 text-indigo-800">{currTimeDisplay}</span>
      <div class="w-full">
        <input
          tabindex="-1"
          on:input={(e) => {
            audio.currentTime = getCurrentTimeByPercentage(
              +e.currentTarget.value,
            );
            updateDisplayTime();
          }}
          value={progress}
          class="h-1 w-full cursor-pointer appearance-none rounded-full bg-indigo-500"
          type="range"
        />
      </div>
      <span class="px-2 text-indigo-800">{totalTimeDisplay}</span>
    </div>
    {#if isReady == 0 && metadata}
      <div class="grid grid-cols-2 gap-2">
        <div>
          {#if selectedBookmark !== undefined}
            <h2 class="text-lg font-semibold">
              Selected Bookmark:{selectedBookmark + 1}
            </h2>
            <textarea
              class="h-96 w-full rounded-lg border border-gray-200 p-4"
              bind:value={bookmarkValue}
              on:keydown={(e) => {
                if (e.ctrlKey && e.key === "Enter") {
                  if (!isPlaying) {
                    rewindAudio();
                  }
                  playPauseAudio();
                } else if (e.shiftKey && e.key === "Tab") {
                  if (!metadata) return;
                  if (selectedBookmark === 0) return;
                  const idx = (selectedBookmark - 1) % metadata.records.length;
                  changeBookmark(idx);
                } else if (e.key === "Tab") {
                  if (!metadata) return;
                  const idx = (selectedBookmark + 1) % metadata.records.length;
                  changeBookmark(idx);
                }
              }}
              bind:this={inputEl}
              on:input={(e) => {
                let value = "";
                if (e?.currentTarget.value) {
                  value = e.currentTarget.value;
                }
                if (metadata) {
                  metadata.records[selectedBookmark].Text = value;
                }
              }}
              on:change={() => {
                saveData();
              }}
            />
            <button
              tabindex="-1"
              class="text-primary-foreground rounded-lg bg-indigo-500 px-4 py-2 text-white"
              on:click={async () => {
                const exportTableValues = metadata?.records
                  .map((record) => {
                    if (
                      record.hidden ||
                      record.Text === undefined ||
                      record.Text === "" ||
                      record.Text === null
                    )
                      return;
                    return "<p>" + record.Text + "</p>";
                  })
                  .filter((e) => e !== undefined);

                if (exportTableValues && exportTableValues.length > 0) {
                  const tempElement = document.createElement("div");
                  tempElement.innerHTML = exportTableValues.join("");

                  // Append to the body
                  document.body.appendChild(tempElement);

                  // Create a range and select the content
                  const range = document.createRange();
                  range.selectNode(tempElement);
                  window.getSelection()?.removeAllRanges();
                  window.getSelection()?.addRange(range);

                  // Copy the selected content
                  try {
                    document.execCommand("copy");
                    copied = true;
                    setTimeout(() => {
                      copied = false;
                    }, 2000);
                  } catch (err) {
                    alert("Failed to copy");
                  }
                  document.body.removeChild(tempElement);
                }
              }}>Export to Notion</button
            >
          {/if}
        </div>
        <div class="pb-4 pt-0">
          <div
            class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
          >
            {#each metadata.records as record, index}
              <div class={getActiveClasses(record, selectedBookmark, index)}>
                <input
                  tabindex="-1"
                  type="checkbox"
                  checked={record.Text !== undefined &&
                    record.Text !== "" &&
                    record.Text !== null}
                  class="pointer-events-none h-4 w-4 rounded border-gray-300 text-indigo-600 accent-indigo-300 focus:ring-indigo-600"
                />
                <button
                  tabindex="-1"
                  on:keydown={() => {
                    audio.currentTime = HMSToSeconds(record.Start);
                    updateTime();
                  }}
                  on:click={() => {
                    changeBookmark(index);
                  }}
                >
                  <div class="font-medium">Chapter:&nbsp;{index + 1}</div>
                  <span class="text-muted-foreground text-sm">
                    Time:&nbsp;{record.Start.split(".")[0]}
                  </span>
                </button>
                <button
                  tabindex="-1"
                  class={record.hidden
                    ? "text-muted-foreground"
                    : selectedBookmark == index
                      ? "text-white"
                      : "text-indigo-800"}
                  on:click={() => {
                    record.hidden = !record.hidden;
                    saveData();
                  }}
                >
                  {#if record.hidden}
                    <svg
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
                        d="M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88"
                      />
                    </svg>
                  {:else}
                    <svg
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
                        d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z"
                      />
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"
                      />
                    </svg>
                  {/if}
                </button>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>
