<script lang="ts">
  const { addTask } = $props();

  async function onSubmit(event: Event) {
    event.preventDefault();
    const form = event.target as HTMLFormElement;
    const input = form.elements.namedItem("task-input") as HTMLInputElement;
    if (!input.value) return;
    addTask(input.value);
    input.value = "";
  }
</script>

<form class="flex items-center gap-2" onsubmit={onSubmit}>
  <label class="label">
    <input
      id="task-input"
      class="input"
      type="text"
      placeholder="Enter a task..."
    />
  </label>

  <button type="submit" class="btn preset-filled">
    <span>&plus;</span>
    <span>Add</span>
  </button>
</form>
