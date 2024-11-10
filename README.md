# Website Carbon Estimator

This is a tool to estimate the carbon emissions of a website using the [CO2.js](https://developers.thegreenwebfoundation.org/co2js/overview/) library. It calculates the carbon footprint of a website visit by analyzing data transfer and energy consumption.

## Usage

First, you need to run the program with the website URL. The default URL is set to [https://www.mogrene.com](https://www.mogrene.com). You can change it in the `src/bin/main.rs` file by modifying the following line:

```
let website_url = "https://www.mogrene.com"; // Change this to the URL you want to check
```

Then, run:

```
cargo run
```


## Output

The amount of data transferred, energy consumption, and carbon emissions.
