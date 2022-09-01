<script>
  import { open } from "@tauri-apps/api/dialog";
  import Modal from "./Modal.svelte";
  import Loading from "./Loading.svelte";
  // import { Button, Modal } from "carbon-components-svelte";
  import AddPersonForm from "./AddPersonForm.svelte";
  // import "carbon-components-svelte/css/all.css";
  // import "carbon-components-svelte/css/g80.css";

  // import Loading from "./Loading.svelte";
  // import { InlineLoading } from "carbon-components-svelte";

  // import { invoke } from '@tauri-apps/api/tauri'
  import { Command } from "@tauri-apps/api/shell";

  let jobs = 0;
  let paths = [];
  let output_job = [];
  let showModal = false;
  let jobInfo = [];

  const handleClick = (e, id) => {
    people = people.filter((person) => person.id != id);
    console.log(e);
  };
  let toggleModal = () => {
    showModal = !showModal;
  };

  const addJob = () => {
    jobs += 1;
  };

  const update_output = (line) => {
    // output_job = [...output_job, line + "\n"];
    document.getElementById("console").value += line + "\n";
    document.getElementById("console").scrollTop =
      document.getElementById("console").scrollHeight;
  };
  const startJobs = async () => {
    let p = 0;
    try {
      for (p in paths) {
        console.log(paths[p]);
        const command = Command.sidecar(
          "/home/jcts/Documents/svelte/json_reader/python_script/dist/test_script",
          paths[p]
        );
        command.stdout.on("data", (line) => update_output(line));
        const output = await command.execute();
        console.log(output);
      }
    } catch (error) {
      console.log(error);
    }
  };

  const readFileContents = async () => {
    try {
      const selectedPath = await open({
        multiple: false,
        title: "Open Json File",
        filters: [
          {
            name: "settings",
            extensions: ["json"],
          },
        ],
      });
      if (selectedPath === null) {
        return;
      }
      paths = [...paths, selectedPath];
      jobInfo = [{ path: selectedPath }, ...jobInfo];
      jobs += 1;
    } catch (error) {
      console.error(error);
    }
  };
</script>

<Modal {showModal} on:click={toggleModal}>
  <AddPersonForm />
</Modal>

<main>
  <!-- <h1>Hello {name}!</h1> -->
  <h1>WF-NTP++</h1>
  <h3>Elegantly tracking Nematodes Even Harder.</h3>
  <button on:click={startJobs}> Start </button>
  <button on:click={readFileContents}> Load Settings file </button>
  <button on:click={toggleModal}>Add Job</button>
  {#each jobInfo as job (job.path)}
    <div>
      {job.path}
    </div>
  {:else}
    <p>There are no Jobs</p>
  {/each}

  <h3>Console Output:</h3>
  <textarea id="console" bind:value={output_job} />
  <!-- <p>Number of jobs: {jobs}</p>
  <p>jobs paths: {paths}</p> -->
  <!-- <Loading /> -->
  <!-- <InlineLoading description="Loading metrics..." /> -->
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    /* background-color: #1a1a1a; */
    cursor: pointer;
    transition: border-color 0.25s;
  }
  button:hover {
    border-color: #646cff;
  }
  button:focus,
  button:focus-visible {
    outline: 4px auto -webkit-focus-ring-color;
  }
  textarea {
    width: 90%;
    height: 50ch;
  }
  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
