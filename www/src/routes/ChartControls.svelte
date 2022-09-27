<script>
    import Chart from 'chart.js/auto/auto.mjs';
    import {SideNavLink} from 'carbon-components-svelte';

    export let outages;
    export let selected;

    let locations = [...outages[1].keys()];
    let data_index = new Map();
    for (let i = 0; i < locations.length; i++) {
        data_index.set(locations[i], i)
    }
    locations.sort();

    function hideAll() {
        const chart = Chart.getChart("myChart");
        chart?.data.datasets.forEach(function(ds) {
            ds.hidden = true;
        });
        chart?.update();
    }

    function showAll() {
        const chart = Chart.getChart("myChart");
        selected = "All Nova Scotia"
        chart?.data.datasets.forEach(function(ds) {
            ds.hidden = false;
        });
        chart?.update();
    }    

    function selectLocation(location) {
        const chart = Chart.getChart("myChart");
        selected = location;
        let index = data_index.get(location);
        hideAll();
        chart.data.datasets[index].hidden = false;
        chart?.update();    
    }
</script>
<SideNavLink text="All Nova Scotia" on:click={showAll} />
{#each locations as location}
    <SideNavLink text="{location}" on:click={() => {selectLocation(location)}} />
{/each}