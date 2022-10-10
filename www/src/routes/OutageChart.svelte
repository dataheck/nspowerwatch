<script lang="ts">
    import Chart from 'chart.js/auto/auto.mjs';
    import 'date-fns';
    import 'chartjs-adapter-date-fns';
    export let outages;

    // https://stackoverflow.com/a/67659413
    let used_colors = new Set();
	function getRandomColour() { //generates random colours and puts them in string
		var letters = '0123456789ABCDEF'.split('');
		
        // ensure unique colours
        var color = '#00a0a0';
        while (used_colors.has(color)) {
            color = "#"
            for (var x = 0; x < 6; x++) {
                color += letters[Math.floor(Math.random() * 16)];
            }
        }
        used_colors.add(color)
	
		return color;
	}
	
    let colormap = new Map();
    
    let datasets: object[] = [];
    outages.forEach((value: object[], key: string, _map: any) => {
        if (!colormap.has(key)) {
            colormap.set(key, getRandomColour())
        } 
        let color = colormap.get(key)
        
        datasets.push({
            data: value,
            label: key,
            borderColor: color,
            backgroundColor: color,
            fill: false,
            radius: 0,
            borderWidth: 2
        })
    });

    let ctx = document.getElementById("myChart");
    const chartInstance = new Chart(ctx, {
        type: "line",
        data: {
            datasets: datasets,
        },        
        options: {
            animation: false,
            parsing: false,
            //normalized: true,
            plugins: {
                legend: {
                    display: false,
                },
                decimation: {
                    enabled: true,
                    algorithm: "lttb", // min-max or lttb
                    samples: 50
                }
            },
            scales: {
                y: {
                    beginAtZero: true
                },
                x: {
                    type: 'time',
                    ticks: {
                        source: 'auto',
                        maxRotation: 0,
                        autoSkip: true,
                    },
                    time: {
                        unit: 'day',
                    }
                }
            },
            responsive: true,
            maintainAspectRatio: false
        },	
    });
</script>