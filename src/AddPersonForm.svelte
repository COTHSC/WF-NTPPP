<script>
  import { open } from "@tauri-apps/api/dialog";
  import { save } from "@tauri-apps/api/dialog";
  import { createDir, BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";

  let name;
  let questions = [
    { id: 1, text: `z filtering` },
    { id: 2, text: `the other one` },
  ];

  let selected;

  var settings = {
    VideoPath: null,
    fps: 0,
    px_to_mm: 0,
    skeletonize: false,
    full_prune: false,
    std_pxl: 0,
    Opening: 0,
    closing: 0,
    threshhold: 100,
  };
  let beltColour;
  let fps = 0;
  let age = 0;
  let yes = false;
  let skeletonize = false;
  let full_prune = false;
  const items = ["One", "Two", "Three"];
  const handleSubmit = () => {};

  const saveDir = async () => {
    const filePath = await save({
      multiple: false,
      saveDir: true,
    });
    console.log(filePath);
    await createDir(filePath, { dir: BaseDirectory.Document, recursive: true });
    let fileName = filePath + "/settings.json";
    await writeTextFile(fileName, JSON.stringify(settings), {
      dir: BaseDirectory.Document,
    });
  };

  const readFileContents = async () => {
    try {
      const selectedPath = await open({
        multiple: false,
        title: "open json file",
      });
      paths = [...paths, selectedPath];
      jobs += 1;
    } catch (error) {
      console.error(error);
    }
  };
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div>
    <h3>Video Settings</h3>
    <button on:click={readFileContents}> Load Video file </button>
  </div>
  <div class="video">
    <input type="text" placeholder="start frame" bind:value={name} />
    <input type="text" placeholder="use frame" bind:value={beltColour} />
    <input type="number" placeholder="fps" bind:value={settings.fps} />
    <input
      type="number"
      placeholder="px to mm"
      bind:value={settings.px_to_mm}
    />
    <label>
      <input type="checkbox" bind:checked={yes} />
      darkfield
    </label>
  </div>
  <h3>Location</h3>
  <div class="locating">
    <input type="text" placeholder="god knows" bind:value={name} />
    <input type="number" placeholder="std pxl" bind:value={settings.std_pxl} />
    <input type="number" placeholder="Opening" bind:value={settings.Opening} />
    <input type="number" placeholder="closing" bind:value={settings.closing} />
    <label>
      <input type="checkbox" bind:checked={skeletonize} />
      skeletonize
    </label>
    <input type="number" placeholder="prune size" bind:value={fps} />
    <label>
      <input type="checkbox" bind:checked={full_prune} />
      full_prune
    </label>
    <!-- <h2>Default</h2> -->
    <!-- <Select {items} /> -->
    <select value={selected}>
      {#each questions as question}
        <option value={question}>
          {question.text}
        </option>
      {/each}
    </select>
  </div>
  <div>
    <button on:click={saveDir}> Output Dir </button>
  </div>
</form>

<style>
  .video {
    display: grid;
    grid-template-columns: repeat(5, 10em);
    grid-template-rows: repeat(4, 3em);
    grid-gap: 0.5em;
  }
  .locating {
    display: grid;
    grid-template-columns: repeat(5, 10em);
    grid-template-rows: repeat(4, 3em);
    grid-gap: 0.5em;
  }
</style>
