const { CO2 } = require('website-carbon-calculator');

async function getCarbonEmission(url) {
    const co2 = new CO2();
    const result = await co2.perVisit(url);
    console.log(JSON.stringify(result));
}

const url = process.argv[2];
getCarbonEmission(url);
