<script>
    import Chart from 'chart.js/auto';
    import 'chartjs-adapter-date-fns';
    
    export let outages;
    let ctx = document.getElementById("myChart").getContext("2d");

    // https://stackoverflow.com/a/67659413
	function getRandomColor() { //generates random colours and puts them in string
		var letters = '0123456789ABCDEF'.split('');
		var color = '#';
		for (var x = 0; x < 6; x++) {
			color += letters[Math.floor(Math.random() * 16)];
		}
	
		return color;
	}
	
    let colormap = new Map();
    let datasets = [];
    outages[1].forEach((value, key, map) => {
        if (!colormap.has(key)) {
            colormap.set(key, getRandomColor())
        } 
        let color = colormap.get(key)
        
        datasets.push({
            data: value,
            label: key,
            borderColor: color,
            backgroundColor: color,
            fill: false,
            hidden: key == "NOVA SCOTIA"
        })
    });

    const chartInstance = new Chart(ctx, {
        type: "line",
        data: {
            labels: outages[0],
            datasets: datasets
        },
        options: {
            scales: {
                x: {
                    type: 'time',
                    time: {
                        unit: 'hour',
                    }
                }
            },
            legend: {
                display: true,
                position: 'top'
            },
            responsive: true,
        },		
    });
</script>