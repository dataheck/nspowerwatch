<script>
    import init, {get_outages} from 'wasm';

	import {
		Header,
		HeaderNav,
		Loading,
		Content,
		SideNav, 
        SideNavItems,
		SideNavLink,
		HeaderNavItem,
	} from 'carbon-components-svelte';
    import "carbon-components-svelte/css/g10.css";

    import Chart from 'chart.js/auto/auto.mjs';
    import OutageChart from './OutageChart.svelte';	
	import { MetaTags } from 'svelte-meta-tags';

    let isSideNavOpen = false;

    function refreshData() {
		const chart = Chart.getChart("myChart");
		if (chart) {
			chart.destroy();
		}
		return get_outages()
	}
</script>

<style>
	canvas {margin-top: 2em;}
</style>

<MetaTags
	title="Nova Scotia Outage Tracker"
	description="Tracks outages across Nova Scotia through time"
	canonical="https://outages.dataheck.com"
	openGraph={{
		url: 'https://outages.dataheck.com',
		title: 'Nova Scotia Outage Tracker',
		description: 'Tracks outages across Nova Scotia through time',
		site_name: 'Data Heck',
		images: [
			{
				url: 'https://outages.dataheck.com/screenshot.png',
			}
		]
	}}
	twitter={{
		handle: '@confirmsignal',
		cardType: 'summary_large_image',
		title: 'Nova Scotia Outage Tracker',
		description: 'Tracks outages across Nova Scotia through time',
	}}
/>

<Header company="Power Outage Time Series" bind:isSideNavOpen>
	<HeaderNav>
		<HeaderNavItem href="https://www.dataheck.com" text="Made with ♥ in Amherst, NS by Matthew Scheffel" />
	</HeaderNav>
</Header>
<SideNav bind:isOpen={isSideNavOpen}>
	<SideNavItems>
		<!--SideNavLink text="Refresh Data" on:click={refreshData} /-->
		<SideNavLink text="Contact" href="https://www.dataheck.com" />
	</SideNavItems>
</SideNav>
<Content>	
	<h1>Nova Scotia Power Outage Tracker</h1>
	<p>
		This site tracks the total number of customers affected by power outages in Nova Scotia as reported
		by <a href="http://outagemap.nspower.ca/external/default.html">Nova Scotia Power</a>. This site is not
		associated with Nova Scotia Power, and this information is provided with no guarantee of its accuracy,
		completeness, reliability, or usefulness.
	</p>
	<br/>
	<p>
		Click a region in the legend below to hide all other series. The largest series at the top will always be 
		all of Nova Scotia. As of this writing, the colours might be similar between different locations. 
	</p>
	<div class="chart-container" style="position: relative; height:80vh; width: 80vw">
		<canvas id="myChart"></canvas>
	</div>
    {#await init()}
        <Loading/>
        {:then}
            {#await refreshData()}
                <Loading/>
            {:then result}
                <OutageChart outages={result} />
            {/await}
    {/await}
</Content>