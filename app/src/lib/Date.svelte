<script lang="ts">
    let dateEvent = new Date();
    let options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };
    let formattedDate = dateEvent.toLocaleDateString('ja', options);
</script> 

<span id=dateString>{formattedDate}</span>
