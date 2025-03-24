<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let newTask = $state("");
  let tasks = $state<{ id: string; text: string; done: boolean }[]>([]);

  onMount(async () => {
    getTasks();
  });

  async function greet(event: Event) {
    event.preventDefault();

    const wasAdded = await invoke("add_task", { newTask });
    console.log({ wasAdded });

    if (wasAdded) {
      getTasks();
    }
  }

  async function getTasks() {
    tasks = await invoke("get_tasks");
  }

  function listen(node: HTMLInputElement, task: { id: string }) {
    async function handleChange(ev: Event) {
      const elm = ev.target as HTMLInputElement;
      console.log(elm.checked);
      const wasCompleted = await invoke("complete_task", {
        taskId: task.id,
        completedBy: "per",
      });
      console.log({ wasCompleted });
    }

    node.addEventListener("change", handleChange);

    return {
      destroy() {
        node.removeEventListener("change", handleChange);
      },
    };
  }
</script>

<main class="w-full h-full container">
  <h1>Eylab Done</h1>
  <p>This is a todo app for Alicia</p>

  <form class="row" onsubmit={greet}>
    <input
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={newTask}
    />
    <button type="submit">Greet</button>
  </form>

  <ul>
    {#each tasks as task}
      <li>
        <label>
          <input type="checkbox" checked={task.done} use:listen={task} />
          {task.text}
        </label>
      </li>
    {/each}
  </ul>
</main>

<style>
</style>
