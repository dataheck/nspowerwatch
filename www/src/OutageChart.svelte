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
        })
    });

    const chartInstance = new Chart(ctx, {
        type: "line",
        data: {
            labels: outages[0],
            datasets: datasets
        },
        options: {
            plugins: {
                legend: {
                    display: true,
                    position: 'bottom',
                    onHover: function(event, legendItem) {
                        document.getElementById("canvas").style.cursor = 'pointer';
                    },
                    onClick: function(e, legendItem) {
                        var index = legendItem.datasetIndex;
                        var ci = this.chart;
                        var alreadyHidden = (ci.getDatasetMeta(index).hidden === null) ? false : ci.getDatasetMeta(index).hidden;

                        ci.data.datasets.forEach(function(e, i) {
                            var meta = ci.getDatasetMeta(i);

                            if (i !== index) {
                            if (!alreadyHidden) {
                                meta.hidden = meta.hidden === null ? !meta.hidden : null;
                            } else if (meta.hidden === null) {
                                meta.hidden = true;
                            }
                            } else if (i === index) {
                            meta.hidden = null;
                            }
                        });

                        ci.update();
                    },
                },
                tooltips: {
                    custom: function(tooltip) {
                    if (!tooltip.opacity) {
                        document.getElementById("canvas").style.cursor = 'default';
                        return;
                    }
                    }
                }, 	
            },
            scales: {
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