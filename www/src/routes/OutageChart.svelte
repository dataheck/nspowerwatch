<script>
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
    
    let datasets = [];
    outages[1].forEach((value, key, map) => {
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
        })
    });

    let ctx = document.getElementById("myChart");
    const chartInstance = new Chart(ctx, {
        type: "line",
        data: {
            labels: outages[0],
            datasets: datasets
        },
        options: {
            /*animation: {
                duration: 0,
            },*/
            plugins: {
                legend: {
                    display: false,
                },
            },
            scales: {
                y: {
                    beginAtZero: true
                },
                x: {
                    type: 'time',
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