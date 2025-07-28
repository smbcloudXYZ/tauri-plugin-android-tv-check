<script>
  import { invoke } from "@tauri-apps/api/core"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands
    greetMsg = await invoke("greet", { name })
  }

  async function check(){
    const isAndroidTv = await invoke("check")
    if (isAndroidTv) {
      greetMsg = "Running on a TV Device"
    } else {
      greetMsg = "Running on a non-TV Device"
    }
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>
