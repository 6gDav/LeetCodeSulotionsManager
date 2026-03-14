import { invoke } from "@tauri-apps/api/core";

const elements: Record<string, HTMLInputElement | null> = {};

const ids = [
  'leet-code-id',
  'leet-code-name',
  'leet-code-icon',
  'programming-language',
  'programming-language-icon',
  'leet-code-url',
  'language-url',
  'solutions-url',
  'description'
];

async function actionHub(elements: Record<string, HTMLInputElement | null>) {

  try {
    await invoke("action_managger", {
      new_id: elements["leet-code-id"]?.value || "",
      leetcode_url: elements["leet-code-url"]?.value || "",
      leetcode_name: elements["leet-code-name"]?.value || "",
      leetcode_icon: elements["leet-code-icon"]?.value || "",
      language_url: elements["language-url"]?.value || "",
      language_name: elements["programming-language"]?.value || "",
      language_icon: elements["programming-language-icon"]?.value || "",
      solution_url: elements["solutions-url"]?.value || "",
      description: elements["description"]?.value || "",
    });
  }
  catch (ex) {
    await invoke("error", {});
  }

}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#greet-form")?.addEventListener("submit", async (e) => {
    e.preventDefault();

    ids.forEach(id => {
      //elements[id] = document.querySelector<HTMLInputElement>(`#${id}`);
      elements[id] = document.querySelector<HTMLInputElement>(`#${id}`);
    });

    try {
      await actionHub(elements);
    }
    catch (ex: any) {
      const dialog = document.getElementById('my-dialog') as HTMLDialogElement;
      const closeBtn = document.getElementById('close-btn') as HTMLButtonElement;
      const errorMessage = document.getElementById('error-massage') as HTMLParagraphElement;

      if (errorMessage)
        errorMessage.textContent = `Error occured: ${ex.message || ex.errorMessage || 'Error'}`;

      dialog.showModal();

      closeBtn.addEventListener('click', () => {
        dialog.close();
      });
    }
  });
});