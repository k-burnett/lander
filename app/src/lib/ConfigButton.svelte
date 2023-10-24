<script lang="ts">
    import ConfigPopup from "./ConfigPopup.svelte";

    function openPopup() {
        let popup = document.getElementById("config-popup");
        if (popup != null) { popup.style.visibility = "visible"; }
    }

</script>

<button class="config-button" on:click={openPopup}>config</button>


<style>
    .config-button {
        background: none;
        border: none;
        text-decoration: underline;
    }

    .config-button:hover {
        background: none;
        border: none;
    }

    .config-button:focus,
    .config-button:focus-visible {
      outline: none;
    }



</style>
