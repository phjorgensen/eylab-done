<script lang="ts">
  const { text, onComplete, onDelete } = $props();
</script>

<section
  class="flex items-center justify-between gap-2 card preset-filled-surface-100-900 border-[1px] border-surface-200-800 w-full p-4"
>
  <p>{text}</p>

  <div class="flex items-center gap-6">
    <button
      type="button"
      class="btn btn-sm preset-tonal-error"
      onclick={onDelete}
    >
      &cross;
    </button>

    <button
      type="button"
      class="btn btn-lg preset-tonal-success"
      onclick={onComplete}
    >
      &check;
    </button>
  </div>
</section>
