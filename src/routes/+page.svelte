<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import TaskCard from "./TaskCard.svelte";
  import TaskForm from "./TaskForm.svelte";

  type TaskData = {
    id: string;
    text: string;
    done: boolean;
    done_at: string;
    done_by: string;
  };

  type Task = ReturnType<typeof createTask>;

  function createTask(taskData: TaskData) {
    return {
      get id() {
        return taskData.id;
      },
      get text() {
        return taskData.text;
      },
      get done() {
        if (taskData.done) {
          return {
            done: true,
            at: taskData.done_at,
            by: taskData.done_by,
          } as const;
        } else {
          return { done: false } as const;
        }
      },
      complete() {
        return invoke<boolean>("complete_task", { taskId: taskData.id })
          .then((res) => res)
          .catch((err) => {
            console.error(err);
            return false;
          });
      },
      delete() {
        return invoke<boolean>("delete_task", { taskId: taskData.id })
          .then((res) => res)
          .catch((err) => {
            console.error(err);
            return false;
          });
      },
    };
  }

  const taskHandler = createTaskHandler();
  onMount(taskHandler.getTasks);

  function createTaskHandler() {
    let tasks = $state<Task[] | undefined>(undefined);

    async function getTasks() {
      const newTasks = await invoke<TaskData[]>("get_tasks");
      tasks = newTasks.map(createTask);
    }

    return {
      get tasks() {
        return tasks;
      },
      getTasks,
      async addTask(newTask: string) {
        const wasAdded = await invoke("add_task", { newTask });
        if (wasAdded) getTasks();
      },
      removeTask(taskId: string) {
        tasks = tasks?.filter((t) => t.id !== taskId);
      },
      complete() {},
      delete() {},
    };
  }
</script>

<main class="container w-full h-full flex flex-col gap-2 px-4 py-6">
  <h1>Eylab Done</h1>

  <TaskForm addTask={taskHandler.addTask} />

  <ul class="flex flex-col gap-1">
    {#each taskHandler.tasks ?? [] as task}
      <li>
        <TaskCard
          text={task.text}
          onComplete={async () => {
            const succ = await task.complete();
            console.log({ succ });
            if (!succ) return;
          }}
          onDelete={async () => {
            const succ = await task.delete();
            console.log({ succ });
            if (!succ) return;
            taskHandler.removeTask(task.id);
          }}
        />
      </li>
    {/each}
  </ul>
</main>

<style>
</style>
