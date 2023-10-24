<script lang="ts">
    
    function closePopup() {
        let popup = document.getElementById("config-popup");
        if (popup != null) { popup.style.visibility = "hidden"; }
    }

    class Pin { 
        private name: string = "";
        private link: string = "";

        constructor(n: string, l: string) {
            this.name = n;
            this.link = l;
        }

        public getName() {
            return this.name;
        }

        public getLink() {
            return this.link;
        }
    } 
    
    let pins: Array<Pin> = [];

    let n: string = "";
    let l: string = "";
    
    function addLink(n: string, l: string) {
        // add the pin to the array 
        pins = [...pins, new Pin(n, l)];

        // then add it to the ui
        // TODO! add method to write pending pins to the screen 
    } 

    function save() {
        console.log(pins);   
        if (pins.length != 0) {
            pins.forEach(element => {
                let pinsDiv = document.getElementById("pins-div");
                if (pinsDiv != null) {
                    pinsDiv.innerHTML += `<a href="${element.getLink()}">${element.getName()}</a>`;
                }
            });
        }
    }

</script>

<div id="config-popup">
    <p>Add a new pin!</p>
    <div id="new-pin-div">
        <input bind:value={n} />
        <input bind:value={l} />
        <button on:click={() => addLink(n, l)}> add </button>
    </div>

    <div id="pins-div">

    </div>

    <button on:click={save}>save</button>
    <button on:click={closePopup}>close</button>
</div>

<style>
    #config-popup {
        visibility: hidden;
        width: 75%;
        height: 75%;
        background: gray;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }
</style>
