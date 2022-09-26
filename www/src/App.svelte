<script>
	export let bindings;

	import Chart from 'chart.js/auto';

	import {
		Header,
		HeaderNav,
		Loading,
		Content,
		ToastNotification,
		SideNav, 
        SideNavItems,
		SideNavLink,
		HeaderNavItem,
	} from 'carbon-components-svelte';

    import OutageChart from './OutageChart.svelte';
	import Meta from 'svelte-meta';

	let isSideNavOpen = false;
	let promise = bindings.get_outages();

	function refreshData() {
		const chart = Chart.getChart("myChart");
		chart.destroy();
		promise = bindings.get_outages();
	}
</script>
<style>
	canvas {
		margin-top: 2em;
	}
</style>
<Meta
	title="Nova Scotia Outage Tracker"
	description="Tracks outages across Nova Scotia through time"
	openGraph={{
		url: 'https://outages.dataheck.com',
		title: "Nova Scotia Outage Tracker",
		description: "Tracks outages across Nova Scotia through time",
		image: 'https://outages.dataheck.com/screenshot.png',
	}}
	twitter={{
		handle: '@confirmsignal',
	}}
/>

<Header company="Power Outage Time Series" bind:isSideNavOpen>
	<HeaderNav>
		<HeaderNavItem href="https://www.dataheck.com" text="Made with ♥ in Amherst, NS by Matthew Scheffel" />
	</HeaderNav>
</Header>
<SideNav bind:isOpen={isSideNavOpen}>
	<SideNavItems>
		<SideNavLink text="Refresh Data" on:click={refreshData} />
		<SideNavLink text="Contact" href="https://www.dataheck.com" />
	</SideNavItems>
</SideNav>
<Content>	
	<h1>Nova Scotia Power Outage Tracker</h1>
	<p>
		This site tracks the total number of customers affected by power outages in Nova Scotia as reported
		by <a href="http://outagemap.nspower.ca/external/default.html">Nova Scotia Power</a>. This site is not
		associated with Nova Scotia Power and this information is provided with no guarantee of its accuracy,
		completeness, reliability, or usefulness.
	</p>
	<p>
		Click a region in the legend below to hide all other series. The largest series at the top will always be 
		all of Nova Scotia. As of this writing, the colours are not gaurenteed to be unique across series; they're 
		randomly generated on each refresh.
	</p>
	<div class="chart-container" style="position: relative; height:80vh; width: 80vw">
		<canvas id="myChart"></canvas>
	</div>
	{#await promise}
		<Loading/>
	{:then result}
		<OutageChart outages={result} />
	{:catch error}
		<ToastNotification
			title="Error"
			subtitle="An internal server error occured."
			caption={error}
		/>
	{/await}
</Content>
