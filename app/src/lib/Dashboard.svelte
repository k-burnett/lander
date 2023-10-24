<script lang="ts">

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
        pins = [...pins, new Pin(n, l)];
    } 

    function save() {
        console.log(pins);   
        if (pins.length != 0) {
            pins.forEach(element => {
                document.getElementById("pins-div").innerHTML += 
                    `<a href="${element.getLink()}">${element.getName()}</a>`;
            });
        }
    }

</script>

<input bind:value={n} />
<input bind:value={l} />

<button on:click={() => addLink(n, l)}> add </button>
<button on:click={save}>save</button>

<div id="pins-div">

</div>